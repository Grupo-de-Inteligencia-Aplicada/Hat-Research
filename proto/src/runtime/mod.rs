mod event_handler;

use crate::home_assistant::HAWebSocket;
use crate::runtime::event_handler::EventHandler;
use anyhow::{Context, Result};
use pest::error::{ErrorVariant, InputLocation, LineColLocation};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;
use tracing::error;

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

pub struct HatRuntime {
    event_handlers: HashMap<String, EventHandler>,
    ha_ws: HAWebSocket,
}

impl HatRuntime {
    pub async fn new(ha_ws_url: &str, ha_token: &str) -> Result<Self> {
        let ha_ws = HAWebSocket::connect(ha_ws_url, ha_token)
            .await
            .context("failed to connect to home assistant")?;

        Ok(Self {
            ha_ws,
            event_handlers: Default::default(),
        })
    }
    pub fn parse(&mut self, filename: String, code: &str) -> Result<(), RuntimeError> {
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
                                Rule::handler_declaration => "handler declaration",
                                Rule::expr => "expression",
                                Rule::handler_condition => "handler condition",
                                Rule::handler_action => "handler action",
                                Rule::stmt => "statement",
                                Rule::program => "program",
                                Rule::handler_triggers => "handler triggers",
                            })
                            .collect(),
                        ErrorVariant::CustomError { .. } => todo!(),
                    },
                })
            }
        };

        println!("{program:#?}");

        Ok(())
    }
}
