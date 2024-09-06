mod echo;

use std::fmt::Debug;

use tokio::runtime::Runtime;

pub use echo::EchoAction;

pub trait Action: Debug {
    fn get_action_name(&self) -> &'static str;
    fn run(&self, runtime: &Runtime);
}
