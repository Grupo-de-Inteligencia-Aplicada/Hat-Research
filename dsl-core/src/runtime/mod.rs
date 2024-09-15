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
use std::future::AsyncDrop;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::task::JoinHandle;
use tracing::{debug, error, warn};

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
    integrations: RwLock<Vec<(Box<dyn Integration>, oneshot::Sender<()>)>>,
    executor_channel: mpsc::UnboundedSender<ExecutorMessage>,
    executor_handle: Mutex<Option<JoinHandle<()>>>,
}

impl HatRuntime {
    pub fn new() -> Arc<Self> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let runtime = Arc::new(Self {
            automations: Default::default(),
            integrations: Default::default(),
            executor_channel: tx,
            executor_handle: Default::default(),
        });

        let runtime_clone = Arc::clone(&runtime);
        let handle = tokio::spawn(async move {
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

        {
            let mut executor_handle = runtime.executor_handle.lock().unwrap();
            *executor_handle = Some(handle);
        }
        
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

    pub async fn integrate<T: 'static + Integration>(&self, integration: T) {
        let mut integration_events = integration.subscribe();
        let integration_name = integration.name();
        let executor_channel = self.executor_channel.clone();
        let (stop_signal_tx, mut stop_signal_rx) = oneshot::channel::<()>();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut stop_signal_rx => {
                        break
                    }
                    event = integration_events.recv() => {
                        if let Some(event) = event {
                            debug!("Event from integration {integration_name}: {event:?}");
                            if executor_channel.send(
                                ExecutorMessage::Event(event)
                            ).is_err() {
                                break;
                            }
                        } else {
                            warn!("Integration {integration_name} closed communication channel before stopping!");
                            break;
                        }
                    },
                }
            }
        });
        let mut integrations = self.integrations.write().await;
        let boxed: Box<dyn Integration> = Box::new(integration);
        integrations.push((boxed, stop_signal_tx));
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
    
    pub async fn join(&self) {
        let mut handle_lock = self.executor_handle.lock().unwrap();
        let handle = &mut *handle_lock;
        if let Some(handle) = handle {
            let _ = handle.await;
        }
    }
}

impl Drop for HatRuntime {
    fn drop(&mut self) {
        warn!("Dropping runtime! Remember to remove all integrations before dropping the runtime!");
    }
}

#[derive(Debug)]
enum ExecutorMessage {
    Event(Event),
}
