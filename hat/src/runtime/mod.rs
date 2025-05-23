pub mod automation;
pub mod context;
pub mod device;
pub mod event;
pub mod function;
pub mod parser;
pub mod scheduler;
pub mod value;

use self::event::Event;
use crate::integrations::clock::ClockIntegration;
use crate::integrations::Integration;
use crate::runtime::automation::Automation;
use crate::runtime::context::ExpressionContext;
use crate::runtime::function::Function;
use anyhow::Result;
use context::Trigger;
use device::Device;
use futures_util::FutureExt;
use scheduler::{ScheduleTask, Scheduler, TaskID};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, Mutex as TokioMutex, RwLock};
use tokio::task::JoinHandle;
use tracing::{error, trace, warn};

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
    #[error("Scheduler error: {inner}")]
    SchedulerError { inner: anyhow::Error },
}

type IntegrationAndStopChannel = (Arc<dyn Integration>, oneshot::Sender<()>);

pub struct HatRuntime {
    scheduler: Scheduler,
    automations: Mutex<HashMap<String, Arc<Automation>>>,
    scheduler_tasks: TokioMutex<HashMap<TaskID, Arc<ScheduleTask>>>,
    integrations: RwLock<HashMap<String, IntegrationAndStopChannel>>,
    executor_channel: mpsc::Sender<ExecutorMessage>,
    executor_handle: tokio::sync::Mutex<Option<JoinHandle<()>>>,
    functions: std::sync::RwLock<HashMap<String, Arc<Function>>>,
}

impl HatRuntime {
    pub async fn new() -> Arc<Self> {
        let (tx, mut rx) = mpsc::channel(128);

        let runtime = Arc::new(Self {
            scheduler: Scheduler::new(tx.clone()).await.unwrap(),
            automations: Default::default(),
            scheduler_tasks: Default::default(),
            integrations: Default::default(),
            executor_channel: tx,
            executor_handle: Default::default(),
            functions: Default::default(),
        });

        runtime.register_default_functions();

        let runtime_clone = Arc::clone(&runtime);
        let handle = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let rt = Arc::clone(&runtime_clone);
                match message {
                    ExecutorMessage::Event(event) => {
                        tokio::spawn(async move {
                            let filtered_automations = {
                                let automations = rt.automations.lock().unwrap();
                                automations
                                    .values()
                                    .filter(|a| a.should_be_triggered_by(&event))
                                    .map(Arc::clone)
                                    .collect::<Vec<_>>()
                            };

                            for automation in filtered_automations {
                                let context = Arc::new(ExpressionContext {
                                    trigger: Trigger::Event(event.clone()),
                                    runtime: Arc::clone(&rt),
                                });
                                if let Err(e) = automation.trigger(context).await {
                                    error!("Failed to run automation {}: {e:?}", automation.name);
                                }
                            }
                        });
                    }
                    ExecutorMessage::TaskRun(task_id) => {
                        if let Some(task) = rt.get_task(&task_id).await {
                            let ctx = Arc::new(ExpressionContext {
                                trigger: Trigger::Task(task_id),
                                runtime: Arc::clone(&rt),
                            });
                            if let Err(e) = task.execute(ctx).await {
                                error!("Failed to run scheduled task {}: {e:?}", task.name);
                            }
                        }
                    }
                }
            }
        });

        {
            let mut executor_handle = runtime.executor_handle.lock().await;
            *executor_handle = Some(handle);
        }

        runtime.integrate(ClockIntegration).await;

        runtime
    }

    pub async fn integrate<T: 'static + Integration>(&self, integration: T) {
        let mut integration_events = integration.subscribe();
        let executor_channel = self.executor_channel.clone();
        let (stop_signal_tx, mut stop_signal_rx) = oneshot::channel::<()>();
        let integration_id = integration.get_id().to_owned();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut stop_signal_rx => {
                        break
                    }
                    event = integration_events.recv() => {
                        if let Some(event) = event {
                            trace!("Event from integration {integration_id}: {event:?}");
                            if executor_channel.send(
                                ExecutorMessage::Event(event)
                            ).await.is_err() {
                                break;
                            }
                        } else {
                            warn!("Integration {integration_id} closed communication channel before stopping!");
                            break;
                        }
                    },
                }
            }
        });
        let integration_arc: Arc<dyn Integration> = Arc::new(integration);
        let mut integrations = self.integrations.write().await;
        integrations.insert(
            integration_arc.get_id().to_owned(),
            (integration_arc, stop_signal_tx),
        );
    }

    pub async fn dispatch_event(&self, event: Event) -> Result<()> {
        self.executor_channel
            .send(ExecutorMessage::Event(event))
            .await?;
        Ok(())
    }

    pub async fn join(&self) {
        let mut handle_lock = self.executor_handle.lock().await;
        let handle = &mut *handle_lock;
        if let Some(handle) = handle {
            let _ = handle.await;
        }
    }

    pub async fn parse(
        &self,
        filename: String,
        code: &str,
    ) -> std::result::Result<(), RuntimeError> {
        let (automations, scheduler_tasks) = parser::parse(filename, code)?;

        {
            let mut automations_lock = self.automations.lock().unwrap();

            for automation in automations {
                let name = automation.name.clone();
                automations_lock.insert(name, Arc::new(automation));
            }
        }

        let mut scheduler_tasks_lock = self.scheduler_tasks.lock().await;

        for task in scheduler_tasks {
            let task = Arc::new(task);
            let task_id = self
                .scheduler
                .schedule(Arc::clone(&task))
                .await
                .map_err(|e| RuntimeError::SchedulerError { inner: e })?;

            scheduler_tasks_lock.insert(task_id, task);
        }

        Ok(())
    }

    pub async fn replace_source(
        &self,
        filename: String,
        code: &str,
    ) -> std::result::Result<(), RuntimeError> {
        self.clear_automations();
        self.clear_scheduler_tasks().await;
        self.parse(filename, code).await
    }

    pub fn clear_automations(&self) {
        let mut lock = self.automations.lock().unwrap();
        lock.clear();
    }

    pub async fn clear_scheduler_tasks(&self) {
        let mut lock = self.scheduler_tasks.lock().await;
        lock.clear();
    }

    pub fn register_function(&self, fun: Function) {
        let mut lock = self.functions.write().unwrap();
        lock.insert(fun.name.clone(), Arc::new(fun));
    }

    pub async fn get_integration(&self, integration: &str) -> Option<Arc<dyn Integration>> {
        let lock = self.integrations.read().await;
        lock.get(integration).map(|(i, _)| Arc::clone(i))
    }

    pub async fn get_integrations(&self) -> Vec<Arc<dyn Integration>> {
        let lock = self.integrations.read().await;
        lock.values().map(|v| Arc::clone(&v.0)).collect()
    }

    /// This function parses a full device ID with the format `{INTEGRATION_ID}@{DEVICE_ID}` or
    /// just `{DEVICE}`.
    /// Returns a tuple in the format: `(integration (if present), device_id)`
    pub fn parse_full_device_id(full_device_id: &str) -> (Option<&str>, &str) {
        if let Some((first, last)) = full_device_id.split_once("@") {
            (Some(first), last)
        } else {
            (None, full_device_id)
        }
    }

    /// This function returns the Device, if it exists, that corresponds to the `device_id`.
    /// The `device_id` can be in the format: `{INTEGRATION_ID}@{DEVICE}`, or just `{DEVICE}`.
    /// On the last option, this function will search on all integrations for a device that matches the `device_id`.
    pub async fn get_device(&self, full_device_id: &str) -> Result<Option<Device>> {
        let (integration, device) = Self::parse_full_device_id(full_device_id);

        if let Some(integration) = integration {
            if let Some(integration) = self.get_integration(integration).await {
                Ok(integration.get_device(device).await?)
            } else {
                Ok(None)
            }
        } else {
            let integrations = self.integrations.read().await;
            for (_, (integration, _)) in integrations.iter() {
                if let Some(device) = integration.get_device(device).await? {
                    return Ok(Some(device));
                }
            }
            Ok(None)
        }
    }

    fn register_default_functions(&self) {
        let mut lock = self.functions.write().unwrap();

        for fun in function::defaults::DEFAULT_FUNCTIONS.iter() {
            lock.insert(fun.name.clone(), Arc::new(fun.clone()));
        }
    }

    async fn get_task(&self, tid: &TaskID) -> Option<Arc<ScheduleTask>> {
        let tasks_lock = self.scheduler_tasks.lock().await;

        tasks_lock.get(tid).map(Arc::clone)
    }
}

impl Drop for HatRuntime {
    fn drop(&mut self) {
        warn!("Dropping runtime! Remember to remove all integrations before dropping the runtime!");
    }
}

#[derive(Debug)]
pub enum ExecutorMessage {
    Event(Event),
    TaskRun(TaskID),
}
