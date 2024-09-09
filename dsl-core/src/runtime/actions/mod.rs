mod echo;

use std::fmt::Debug;

pub use echo::EchoAction;

use super::HatRuntime;

pub trait Action: Debug + Send + Sync {
    fn get_action_name(&self) -> &'static str;
    fn run(&self, runtime: &HatRuntime);
}
