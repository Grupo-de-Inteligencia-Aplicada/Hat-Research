pub mod actions;
pub mod automation;
pub mod context;
pub mod device;
pub mod event;
pub mod expression;
pub mod function;
pub mod value;

use self::event::Event;
use crate::integrations::Integration;
use crate::runtime::automation::Automation;
use crate::runtime::context::AutomationContext;
use crate::runtime::expression::Expression;
use crate::runtime::function::FunctionCall;
use actions::{Action, EchoAction};
use anyhow::{bail, Context, Result};
use pest::error::{ErrorVariant, InputLocation, LineColLocation};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::task::JoinHandle;
use tracing::{debug, error, warn};

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

type IntegrationAndStopChannel = (Box<dyn Integration>, oneshot::Sender<()>);

pub struct HatRuntime {
    automations: Mutex<HashMap<String, Automation>>,
    integrations: RwLock<Vec<IntegrationAndStopChannel>>,
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
                                let mut context = AutomationContext {
                                    event: event.clone(),
                                };
                                if let Err(e) = automation.trigger(&runtime_clone, &mut context) {
                                    error!("Failed to run automation {}: {e:?}", automation.name);
                                }
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
        // TODO: stop panicking
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
                                Rule::const_expr => "constant expression",
                                Rule::bool_expr => "boolean expression",
                                Rule::function_expr => "function expression",
                                Rule::function_parameters => "function parameters",
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
                let name = match name_rule.as_rule() {
                    Rule::ident => name_rule.as_span().as_str().to_owned(),
                    Rule::string => Self::parse_string(name_rule).expect("failed to parse string"),
                    _ => unreachable!(),
                };

                let triggers: Vec<_> = inner
                    .next()
                    .expect("missing the automation triggers")
                    .into_inner()
                    .map(|trigger| trigger.as_span().as_str().to_owned())
                    .collect();

                let mut maybe_conditions_or_actions =
                    inner.next().expect("missing the automation action");

                let mut conditions: Vec<Expression> = Vec::new();

                if maybe_conditions_or_actions.as_rule() == Rule::automation_conditions {
                    conditions = maybe_conditions_or_actions
                        .into_inner()
                        .map(|r| Self::parse_expression(r).unwrap())
                        .collect();

                    maybe_conditions_or_actions =
                        inner.next().expect("missing the automation action");
                }

                let actions = maybe_conditions_or_actions
                    .into_inner()
                    .map(|r| Self::parse_action(r).unwrap())
                    .collect::<Vec<_>>();

                let automation = Automation {
                    name: name.clone(),
                    triggers,
                    conditions,
                    actions,
                };

                automations.insert(name, automation);
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

    pub async fn join(&self) {
        let mut handle_lock = self.executor_handle.lock().unwrap();
        let handle = &mut *handle_lock;
        if let Some(handle) = handle {
            let _ = handle.await;
        }
    }

    fn parse_string(rule: Pair<Rule>) -> Result<String> {
        match rule.as_rule() {
            Rule::string => {
                let val = rule.as_span().as_str();
                Ok(val[1..val.len() - 1].to_owned())
            }
            _ => bail!("rule is not a string"),
        }
    }

    fn parse_action(rule: Pair<Rule>) -> Result<Box<dyn Action>> {
        match rule.as_rule() {
            Rule::echo_action => {
                let message = rule.into_inner().next().unwrap().as_span().as_str();
                Ok(Box::new(EchoAction::new(
                    message[1..message.len() - 1].to_owned(),
                )))
            }
            _ => bail!("rule is not an action"),
        }
    }

    fn parse_expression(rule: Pair<Rule>) -> Result<Expression> {
        match rule.as_rule() {
            Rule::expr => {
                let inner = rule
                    .into_inner()
                    .next()
                    .context("expression does not have inner rules")?;

                match inner.as_rule() {
                    Rule::const_expr => {
                        let inner = inner
                            .into_inner()
                            .next()
                            .context("constant expression does not have inner rules")?;
                        match inner.as_rule() {
                            Rule::bool_expr => match inner.as_span().as_str() {
                                "true" => Ok(Expression::Constant(true.into())),
                                "false" => Ok(Expression::Constant(false.into())),
                                _ => unreachable!(),
                            },
                            Rule::string => {
                                Self::parse_string(inner).map(|s| Expression::Constant(s.into()))
                            }
                            Rule::decimal => {
                                let inner = inner.as_span().as_str();
                                Ok(Expression::Constant(f64::from_str(inner)?.into()))
                            }
                            Rule::integer => {
                                let inner = inner.as_span().as_str();
                                Ok(Expression::Constant((i64::from_str(inner)? as f64).into()))
                            }
                            _ => {
                                unimplemented!()
                            }
                        }
                    }
                    Rule::function_expr => {
                        let mut inner = inner.into_inner();
                        let name = inner
                            .next()
                            .context("function expression does not have inner rules")?
                            .as_span()
                            .as_str();
                        let parameters = inner
                            .next()
                            .context("function expression have just one inner rule")?
                            .into_inner()
                            .map(|rule| Self::parse_expression(rule))
                            .collect::<Result<Vec<_>>>()?;
                        Ok(Expression::Function(FunctionCall {
                            name: name.to_owned(),
                            arguments: parameters,
                        }))
                    }
                    _ => bail!("unknown expression rule"),
                }
            }
            _ => bail!("rule is not an expression"),
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
