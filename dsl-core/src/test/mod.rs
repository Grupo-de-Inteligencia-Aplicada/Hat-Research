use crate::runtime::context::AutomationContext;
use crate::runtime::device::{Device, DeviceType};
use crate::runtime::event::{Event, EventType};
use crate::runtime::parser::expression::Expression;
use crate::runtime::function::FunctionCall;
use crate::runtime::value::Value;

#[test]
pub fn test_parse_sample() {
    let src = include_str!("sample.hat");
}

#[test]
pub fn test_function_call() {
    let expression: Expression = FunctionCall {
        name: "get_device".to_string(),
        arguments: vec![
            FunctionCall {
                name: "get_integration".to_string(),
                arguments: vec![],
            }
            .into(),
            Value::Boolean(true).into(),
        ],
    }
    .into();

    let mut context = AutomationContext {
        event: Event {
            typ: EventType::Dummy,
            time: Default::default(),
            device: Device {
                integration: "TestIntegration".to_string(),
                id: "test_device".to_string(),
                name: None,
                typ: DeviceType::Dummy,
            },
            parameters: Default::default(),
        },
    };

    let result = expression.evaluate(&mut context).unwrap();

    assert_eq!(result, Value::String(context.event.device.full_id()))
}
