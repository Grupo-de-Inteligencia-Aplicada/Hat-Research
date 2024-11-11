use anyhow::{bail, Context, Result};
use axum::{extract::State, response::IntoResponse};
use http::StatusCode;
use tracing::debug;

use crate::runtime::{automation::Automation, parser::expression::Expression, value::Value};

use super::AppState;
use blocks::{Block, XmlWorkspace};

mod blocks;

/// This function will convert a Blockly XML workspace into Hat source code
#[axum::debug_handler]
pub(super) async fn transpile_workspace_to_hat(
    State(_): State<AppState>,
    xml: String,
) -> impl IntoResponse {
    // TODO: remove unwraps
    let document: XmlWorkspace = quick_xml::de::from_str(&xml).unwrap();

    let automations = document
        .blocks
        .into_iter()
        .map(|block| {
            let mut conditions = Vec::new();
            let mut actions = Vec::new();
            let mut triggers = Vec::new();
            if let Some(event) = block.get_value("EVENT") {
                triggers.push(
                    event
                        .inner_block
                        .block_type
                        .clone()
                        .split_off("event_".len()),
                );
            }

            if let Some(body) = block.get_statement("BODY") {
                let mut next_block: Option<Block> = Some(body.inner_block.clone());
                while let Some(next) = next_block {
                    match next.block_type.as_str() {
                        "condition" => {
                            let condition_expr_block =
                                next.get_value("CONDITION").unwrap().inner_block.clone();
                            let expr = parse_expression_block(condition_expr_block).unwrap();
                            conditions.push(expr);
                        }
                        "action" => {
                            let action_expr_block =
                                next.get_value("ACTION").unwrap().inner_block.clone();
                            let expr = parse_expression_block(action_expr_block).unwrap();
                            actions.push(expr);
                        }
                        _ => {}
                    }
                    next_block = next.next.map(|n| n.inner_block);
                }
            }

            Automation {
                name: block.get_field("NAME").unwrap().to_owned(),
                triggers,
                conditions,
                actions,
            }
        })
        .collect::<Vec<_>>();

    let code = generate_hat_code(automations);

    debug!("{code}");
    (StatusCode::OK, code.to_string())
}

/// This function will convert Hat source code into a Blockly XML workspace
pub(super) async fn transpile_hat_to_workspace(State(_): State<AppState>, src: String) -> String {
    todo!()
}

pub fn parse_expression_block(block: Block) -> Result<Expression> {
    match block.block_type.as_str() {
        // Constants
        "const_string" => Ok(Expression::Constant(Value::String(
            block
                .get_field("VALUE")
                .context("missing string value")?
                .clone(),
        ))),
        _ => bail!("unknown block type"),
    }
}

pub fn generate_hat_code(automations: Vec<Automation>) -> String {
    let mut buffer = String::new();

    for automation in automations {
        let conditions = automation
            .conditions
            .into_iter()
            .map(|expr| expr.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let actions = automation
            .actions
            .into_iter()
            .map(|expr| format!("run {expr}"))
            .collect::<Vec<_>>()
            .join("\n");

        buffer.push_str(&format!(
            r#"
                automation \"{}\" ({}) {{
                    {}{}
                }}
                "#,
            automation.name,
            automation.triggers.join(", "),
            conditions,
            actions,
        ));
    }

    buffer
}
