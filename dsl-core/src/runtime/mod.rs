pub mod actions;
pub mod automation;
pub mod device;
pub mod event;

use crate::integrations::Integration;
use crate::runtime::automation::Automation;
use actions::{Action, EchoAction};
use anyhow::Result;
use pest::error::{ErrorVariant, InputLocation, LineColLocation};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::sync::mpsc;
use tracing::error;

use self::event::Event;

#[derive(Parser)]
#[grammar = "grammars/hat.pest"]
pub struct DcParser;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error(
        "\
        Syntax error at {file}:{line_number}:{col_number}. Expected: {}\
        \nAt: {line}
        ",
        expected.join(", "))
    ]
    ParseError {
        file: String,
        line: String,
        location_start: usize,
        location_end: usize,
        line_number: usize,
        col_number: usize,
        expected: Vec<&'static str>,
    },
}

pub struct HatRuntime {
    automations: Mutex<HashMap<String, Automation>>,
    integrations: HashSet<Box<dyn Integration>>,
    executor_channel: mpsc::UnboundedSender<ExecutorMessage>,
}

impl HatRuntime {
    pub fn new() -> Arc<Self> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let runtime = Arc::new(Self {
            automations: Default::default(),
            integrations: HashSet::new(),
            executor_channel: tx,
        });

        let runtime_clone = Arc::clone(&runtime);
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                match message {
                    ExecutorMessage::Event(event) => {
                        let automations = runtime_clone.automations.lock().unwrap();

                        for (_name, automation) in automations.iter() {
                            if automation.should_be_triggered_by(&event) {
                                automation.trigger(&runtime_clone);
                            }
                        }
                    }
                }
            }
        });

        runtime
    }

    pub fn parse(&self, filename: String, code: &str) -> Result<(), RuntimeError> {
        let code_program = DcParser::parse(Rule::program, code);

        let program = match code_program {
            Ok(program) => program,
            Err(e) => {
                return Err(RuntimeError::ParseError {
                    file: filename,
                    line: e.line().to_owned(),
                    location_start: match e.location {
                        InputLocation::Pos(x) => x,
                        InputLocation::Span((x, _)) => x,
                    },
                    location_end: match e.location {
                        InputLocation::Pos(x) => x,
                        InputLocation::Span((_, y)) => y,
                    },
                    line_number: match e.line_col {
                        LineColLocation::Pos((x, _)) => x,
                        LineColLocation::Span((x, _), _) => x,
                    },
                    col_number: match e.line_col {
                        LineColLocation::Pos((_, y)) => y,
                        LineColLocation::Span((_, y), _) => y,
                    },
                    expected: match e.variant {
                        ErrorVariant::ParsingError {
                            positives,
                            negatives: _,
                        } => positives
                            .into_iter()
                            .map(|rule| match rule {
                                Rule::EOI => "end of input",
                                Rule::COMMENT => "comment",
                                Rule::SINGLE_LINE_COMMENT => "single line comment",
                                Rule::BLOCK_COMMENT => "block comment",
                                Rule::WHITESPACE => "whitespace",
                                Rule::ident => "identifier",
                                Rule::integer => "integer value",
                                Rule::decimal => "decimal value",
                                Rule::string => "string value",
                                Rule::event_declaration => "event declaration",
                                Rule::event_parameter => "event parameter",
                                Rule::event_parameters => "event parameter list",
                                Rule::type_keyword => "type",
                                Rule::automation_declaration => "automation declaration",
                                Rule::expr => "expression",
                                Rule::automation_conditions => "automation condition",
                                Rule::stmt => "statement",
                                Rule::program => "program",
                                Rule::automation_triggers => "automation triggers",
                                Rule::echo_action => "echo command",
                                Rule::automation_actions => "automation actions",
                            })
                            .collect(),
                        ErrorVariant::CustomError { .. } => todo!(),
                    },
                })
            }
        };

        let mut automations = self.automations.lock().unwrap();

        for rule in program {
            if matches!(rule.as_rule(), Rule::automation_declaration) {
                let mut inner = rule.into_inner();

                let name_rule = inner.next().expect("missing name of the automation");
                let name_str = name_rule.as_span().as_str();
                let name = match name_rule.as_rule() {
                    Rule::ident => name_str,
                    Rule::string => &name_str[1..name_str.len() - 1],
                    _ => unreachable!(),
                };

                let triggers: Vec<_> = inner
                    .next()
                    .expect("missing the automation triggers")
                    .into_inner()
                    .map(|trigger| trigger.as_span().as_str().to_owned())
                    .collect();

                let actions = inner
                    .next()
                    .expect("missing the automation action")
                    .into_inner()
                    .filter_map(|r| Self::parse_action(r))
                    .collect::<Vec<_>>();

                let automation = Automation {
                    name: name.to_owned(),
                    triggers,
                    actions,
                };

                automations.insert(name.to_owned(), automation);
            }
        }

        Ok(())
    }

    pub fn dispatch_event(&self, event: Event) -> Result<()> {
        self.executor_channel.send(ExecutorMessage::Event(event))?;
        Ok(())
    }

    fn parse_action(rule: Pair<Rule>) -> Option<Box<dyn Action>> {
        match rule.as_rule() {
            Rule::echo_action => {
                let message = rule.into_inner().next().unwrap().as_span().as_str();
                Some(Box::new(EchoAction::new(
                    message[1..message.len() - 1].to_owned(),
                )))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
enum ExecutorMessage {
    Event(Event),
}
