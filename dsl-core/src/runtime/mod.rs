pub mod actions;
pub mod automation;
pub mod context;
pub mod device;
pub mod event;
pub mod function;
pub mod value;
pub mod parser;

use self::event::Event;
use crate::integrations::Integration;
use crate::runtime::automation::Automation;
use crate::runtime::context::AutomationContext;
use parser::expression::Expression;
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

    pub fn parse(&self, filename: String, code: &str) -> std::result::Result<(), RuntimeError> {
        parser::parse(self, filename, code)
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
