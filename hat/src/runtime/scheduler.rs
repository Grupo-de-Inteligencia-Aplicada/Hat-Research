use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use chrono::Timelike;
use tokio::sync::mpsc;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tracing::debug;
use uuid::Uuid;

use super::{
    context::{ExpressionContext, Trigger},
    parser::expression::Expression,
    value::time::Time,
    ExecutorMessage,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskID(pub Uuid);

pub struct Scheduler {
    inner_scheduler: JobScheduler,
    runtime_msg_tx: mpsc::UnboundedSender<ExecutorMessage>,
}

impl Scheduler {
    pub async fn new(runtime_msg_tx: mpsc::UnboundedSender<ExecutorMessage>) -> Result<Self> {
        let inner = JobScheduler::new().await?;
        inner.start().await?;
        Ok(Self {
            inner_scheduler: inner,
            runtime_msg_tx,
        })
    }

    pub async fn schedule(&self, task: Arc<ScheduleTask>) -> Result<TaskID> {
        let executor_tx = self.runtime_msg_tx.clone();
        let cron_expr = task.interval.as_cron_expr();
        debug!("Scheduling {} with cron expr {cron_expr}...", task.name);
        let id = self
            .inner_scheduler
            .add(
                Job::new_async_tz(&cron_expr, chrono::Local, move |tid, _l| {
                    let tx = (&executor_tx).clone();
                    Box::pin(async move { tx.send(ExecutorMessage::TaskRun(TaskID(tid))).unwrap() })
                })
                .map_err(|e| match e {
                    JobSchedulerError::ParseSchedule => {
                        anyhow!("failed to parse cron expression: {cron_expr}")
                    }
                    _ => anyhow!(e),
                })?,
            )
            .await?;

        Ok(TaskID(id))
    }
}

#[derive(Debug)]
pub struct ScheduleTask {
    pub name: String,
    pub interval: ScheduleInterval,
    pub conditions: Vec<Expression>,
    pub actions: Vec<Expression>,
}

impl ScheduleTask {
    pub async fn execute(&self, ctx: Arc<ExpressionContext>) -> Result<()> {
        for condition in &self.conditions {
            let result = condition
                .evaluate(Arc::clone(&ctx))
                .await
                .with_context(|| {
                    format!("failed to evaluate expression in condition {condition:?}")
                })?;

            if !result.as_bool() {
                return Ok(());
            }
        }
        for action in &self.actions {
            action
                .evaluate(Arc::clone(&ctx))
                .await
                .with_context(|| format!("action of automation {} failed", self.name))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ScheduleInterval {
    Cron(String),
    Time { weekday: Option<Weekday>, at: Time },
}

impl ScheduleInterval {
    pub fn as_cron_expr(&self) -> String {
        match self {
            Self::Cron(e) => e.clone(),
            Self::Time { weekday, at } => {
                let sec = at.second();
                let minute = at.minute();
                let hour = at.hour();
                let dom = "*";
                let month = "*";
                // Sunday = 0 … Saturday = 6
                let dow = weekday
                    .as_ref()
                    .map(|w| w.num_days_from_sunday().to_string())
                    .unwrap_or_else(|| "*".into());

                // six-field cron: "SEC MIN HOUR DOM MON DOW"
                format!("{} {} {} {} {} {}", sec, minute, hour, dom, month, dow)
            }
        }
    }
}

#[derive(Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn num_days_from_sunday(&self) -> u8 {
        match self {
            Self::Sunday => 0,
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
        }
    }
}

impl TryFrom<&str> for Weekday {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "monday" => Ok(Self::Monday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),
            "thursday" => Ok(Self::Thursday),
            "friday" => Ok(Self::Friday),
            "saturday" => Ok(Self::Saturday),
            "sunday" => Ok(Self::Sunday),
            _ => Err(()),
        }
    }
}
