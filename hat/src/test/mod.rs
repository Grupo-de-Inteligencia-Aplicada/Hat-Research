use std::sync::Arc;
use crate::runtime::context::AutomationContext;
use crate::runtime::device::{Device, DeviceType};
use crate::runtime::event::{Event, EventType};
use crate::runtime::function::FunctionCall;
use crate::runtime::HatRuntime;
use crate::runtime::parser::expression::Expression;
use crate::runtime::parser::expression::Expression::{BinaryOperation, Constant, Function};
use crate::runtime::parser::operation::Operation;
use crate::runtime::value::Value;

#[tokio::test]
pub async fn test_parse_sample() {
    async fn parse_sample(src: &str) {
        let runtime = HatRuntime::new().await;
        runtime.parse("test.hat".into(), src).unwrap();
    }

    parse_sample(include_str!("sample.hat")).await;
    parse_sample(include_str!("another.hat")).await;
}

#[tokio::test]
pub async fn test_function_call() {
    let runtime = HatRuntime::new().await;

    let context = AutomationContext {
        event: Event {
            typ: EventType::Dummy,
            datetime: Default::default(),
            device: Device {
                integration: "test".to_string(),
                id: "test_dev".to_string(),
                name: None,
                typ: DeviceType::Dummy,
                state: None,
                attributes: Default::default(),
            },
            parameters: Default::default(),
        },
        runtime: Arc::clone(&runtime),
    };

    let expression: Expression = BinaryOperation {
        lhs: Box::new(Constant(Value::String("Example-".into()))),
        op: Operation::Add,
        rhs: Box::new(Function(FunctionCall {
            name: "get_device".to_string(),
            arguments: vec![],
        })),
    };

    let result = expression.evaluate(Arc::new(context)).await.unwrap();

    assert_eq!(result, Value::String("Example-test@test_dev".into()));
}
