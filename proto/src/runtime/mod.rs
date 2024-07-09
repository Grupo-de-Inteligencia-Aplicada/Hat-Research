mod event_handler;

use std::collections::HashMap;
use pest::error::{ErrorVariant, InputLocation, LineColLocation};
use pest::Parser;
use thiserror::Error;
use pest_derive::Parser;
use crate::runtime::event_handler::EventHandler;

#[derive(Parser)]
#[grammar = "grammars/hat.pest"]
pub struct DcParser;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Syntax error at {file}:{line_number}:{col_number}. Expected: {}", expected.join(", "))]
    ParseError {
        file: String,
        line: String,
        location_start: usize,
        location_end: usize,
        line_number: usize,
        col_number: usize,
        expected: Vec<&'static str>,
    }
}

#[derive(Default)]
pub struct HatRuntime {
    event_handlers: HashMap<String, EventHandler>,
}

impl HatRuntime {
    pub fn parse(&mut self, filename: String, code: &str) -> Result<(), RuntimeError> {
        let code_program = DcParser::parse(Rule::program, code);

        let program = match code_program {
            Ok(program) => program,
            Err(e) => {
                return Err(RuntimeError::ParseError {
                    file: filename,
                    line: "".to_string(),
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
                        ErrorVariant::ParsingError { positives, negatives: _ } => {
                            positives.into_iter().map(|rule| match rule {
                                Rule::EOI => "end of input",
                                Rule::whitespace => "whitespace",
                                Rule::ident => "identifier",
                                Rule::integer => "integer value",
                                Rule::decimal => "decimal value",
                                Rule::string => "string value",
                                Rule::event_declaration => "event declaration",
                                Rule::event_parameter => "event parameter",
                                Rule::event_parameters => "event parameter list",
                                Rule::type_keyword => "type",
                                Rule::handler_declaration => "handler declaration",
                                Rule::handler_expr => "handler expression",
                                Rule::expr => "expression",
                                Rule::handler_condition => "handler condition",
                                Rule::handler_action => "handler action",
                                Rule::stmt => "statement",
                                Rule::program => "program",
                            }).collect()
                        },
                        ErrorVariant::CustomError { .. } => todo!(),
                    },
                })
            }
        };

        println!("{program:#?}");

        Ok(())
    }
}
