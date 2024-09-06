use super::actions::Action;

pub struct Automation {
    pub name: String,
    pub triggers: Vec<String>,
    pub actions: Vec<Box<dyn Action>>,
}
