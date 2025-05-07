use crate::runtime::automation::Automation;
use crate::runtime::function::FunctionCall;
use crate::runtime::scheduler::Weekday;
use crate::runtime::value::Value;
use crate::runtime::RuntimeError;
use anyhow::{bail, Context, Result};
use expression::Expression;
use operation::Operation;
use pest::error::{ErrorVariant, InputLocation, LineColLocation};
use pest::iterators::{Pair, Pairs};
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;

use super::scheduler::{ScheduleInterval, ScheduleTask};
use super::value::time::Time;

pub mod expression;
pub mod operation;

#[derive(Parser)]
#[grammar = "grammars/hat.pest"]
struct HatParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(equals, Left) | Op::infix(not_equals, Left))
            .op(Op::infix(and, Left) | Op::infix(or, Left))
            .op(Op::infix(greater, Left) | Op::infix(greater_eq, Left) | Op::infix(lesser, Left) | Op::infix(lesser_eq, Left))
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    };
}

pub fn parse(
    filename: String,
    code: &str,
) -> std::result::Result<(Vec<Automation>, Vec<ScheduleTask>), RuntimeError> {
    // TODO: stop panicking
    let code_program = HatParser::parse(Rule::program, code);

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
                            Rule::automation_declaration => "automation declaration",
                            Rule::expr => "expression",
                            Rule::automation_condition => "automation condition",
                            Rule::stmt => "statement",
                            Rule::program => "program",
                            Rule::automation_triggers => "automation triggers",
                            Rule::automation_action => "automation action",
                            Rule::const_atom => "constant",
                            Rule::bool => "boolean",
                            Rule::function => "function",
                            Rule::function_parameters => "function parameters",
                            Rule::atom => "value (atom)",
                            Rule::bin_op => "binary operation",
                            Rule::add => "addition (+)",
                            Rule::subtract => "addition (-)",
                            Rule::multiply => "addition (*)",
                            Rule::divide => "addition (/)",
                            Rule::null => "null",
                            Rule::equals => "==",
                            Rule::not_equals => "!=",
                            Rule::and => "and",
                            Rule::or => "or",
                            Rule::greater => ">",
                            Rule::greater_eq => ">=",
                            Rule::lesser => "<",
                            Rule::lesser_eq => "<=",
                            Rule::schedule_interval => "schedule interval",
                            Rule::time => "time",
                            Rule::schedule_declaration => "schedule declaration",
                            Rule::schedule_interval_time => "schedule interval time",
                            Rule::schedule_interval_time_weekday => {
                                "weekday of the schedule interval"
                            }
                            Rule::schedule_interval_cron => "schedule interval cron",
                        })
                        .collect(),
                    ErrorVariant::CustomError { .. } => todo!(),
                },
            })
        }
    };

    let mut automations = Vec::new();
    let mut scheduler_tasks = Vec::new();

    for rule in program {
        match rule.as_rule() {
            Rule::automation_declaration => {
                let mut inner = rule.into_inner();

                let name_rule = inner.next().expect("missing name of the automation");
                let name = match name_rule.as_rule() {
                    Rule::ident => name_rule.as_span().as_str().to_owned(),
                    Rule::string => parse_string(name_rule).expect("failed to parse string"),
                    _ => unreachable!(),
                };

                let triggers: Vec<_> = inner
                    .next()
                    .expect("missing the automation triggers")
                    .into_inner()
                    .map(|trigger| trigger.as_span().as_str().to_owned())
                    .collect();

                let mut conditions = Vec::new();
                let mut actions = Vec::new();

                for next in inner {
                    match next.as_rule() {
                        Rule::automation_condition => {
                            conditions.push(parse_expression(next.into_inner()).unwrap());
                        }
                        Rule::automation_action => {
                            actions.push(parse_expression(next.into_inner()).unwrap());
                        }
                        _ => unreachable!(),
                    }
                }

                let automation = Automation {
                    name: name.clone(),
                    triggers,
                    conditions,
                    actions,
                };

                automations.push(automation);
            }
            Rule::schedule_declaration => {
                let mut inner = rule.into_inner();

                let name_rule = inner.next().expect("missing name of the automation");
                let name = match name_rule.as_rule() {
                    Rule::ident => name_rule.as_span().as_str().to_owned(),
                    Rule::string => parse_string(name_rule).expect("failed to parse string"),
                    _ => unreachable!(),
                };

                let interval = inner
                    .next()
                    .expect("missing schedule interval")
                    .into_inner()
                    .next()
                    .unwrap();

                let interval = match interval.as_rule() {
                    Rule::schedule_interval_cron => {
                        let inner = interval.into_inner().next().unwrap();
                        let cron_rule =
                            parse_string(inner).expect("failed to parse cron rule string");

                        ScheduleInterval::Cron(cron_rule)
                    }
                    Rule::schedule_interval_time => {
                        let mut inner = interval.into_inner();
                        let mut weekday: Option<Weekday> = None;

                        if inner.len() > 1 {
                            weekday =
                                Some(inner.next().unwrap().as_span().as_str().try_into().unwrap());
                        }

                        let time_span = inner.next().unwrap().as_span().as_str();
                        let time = parse_time(time_span)
                            .expect(&format!("invalid time format: {time_span}"));

                        ScheduleInterval::Time { weekday, at: time }
                    }
                    _ => unreachable!(),
                };

                let mut conditions = Vec::new();
                let mut actions = Vec::new();

                for next in inner {
                    match next.as_rule() {
                        Rule::automation_condition => {
                            conditions.push(parse_expression(next.into_inner()).unwrap());
                        }
                        Rule::automation_action => {
                            actions.push(parse_expression(next.into_inner()).unwrap());
                        }
                        _ => unreachable!(),
                    }
                }

                let schedule_task = ScheduleTask {
                    name,
                    interval,
                    conditions,
                    actions,
                };

                scheduler_tasks.push(schedule_task);
            }
            Rule::EOI => {}
            _ => unreachable!("top level rule not implemented {rule:?}"),
        }
    }

    Ok((automations, scheduler_tasks))
}

pub fn parse_time(span: &str) -> Result<Time> {
    let mut parts = span.split(":");
    let hours: u32 = parts
        .next()
        .context("time string is empty")?
        .parse()
        .context("failed to parse hours")?;
    let mins: u32 = parts
        .next()
        .map(|s| s.parse::<u32>())
        .unwrap_or(Ok(0))
        .context("failed to parse minutes")?;
    let secs: u32 = parts
        .next()
        .map(|s| s.parse::<u32>())
        .unwrap_or(Ok(0))
        .context("failed to parse seconds")?;
    Ok(Time::from_hms_opt(hours, mins, secs).context("invalid time provided")?)
}

fn parse_string(rule: Pair<Rule>) -> Result<String> {
    match rule.as_rule() {
        Rule::string => {
            let val = rule.as_span().as_str();
            Ok(val[1..val.len() - 1].to_owned())
        }
        _ => bail!("rule is not a string"),
    }
}

fn parse_atom(rule: Pair<Rule>) -> Result<Expression> {
    match rule.as_rule() {
        Rule::atom => {
            let inner = rule.into_inner().next().context("empty atom")?;
            match inner.as_rule() {
                Rule::null => Ok(Expression::Constant(Value::Null)),
                Rule::bool => match inner.as_span().as_str() {
                    "true" => Ok(Expression::Constant(true.into())),
                    "false" => Ok(Expression::Constant(false.into())),
                    _ => unreachable!(),
                },
                Rule::string => parse_string(inner).map(|s| Expression::Constant(s.into())),
                Rule::time => parse_time(inner.as_span().as_str())
                    .map(|time| Expression::Constant(time.into())),
                Rule::decimal => {
                    let inner = inner.as_span().as_str();
                    Ok(Expression::Constant(f64::from_str(inner)?.into()))
                }
                Rule::integer => {
                    let inner = inner.as_span().as_str();
                    Ok(Expression::Constant((i64::from_str(inner)? as f64).into()))
                }
                Rule::function => {
                    let mut inner = inner.into_inner();
                    let name = inner
                        .next()
                        .context("function expression does not have inner rules")?
                        .as_span()
                        .as_str();
                    let parameters = inner
                        .next()
                        .context("function expression have just one inner rule")?
                        .into_inner()
                        .map(|rule| parse_expression(rule.into_inner()))
                        .collect::<Result<Vec<_>>>()?;
                    Ok(Expression::Function(FunctionCall {
                        name: name.to_owned(),
                        arguments: parameters,
                    }))
                }
                Rule::expr => parse_expression(inner.into_inner()),
                _ => bail!("unknown atom rule: {inner:?}"),
            }
        }
        _ => bail!("unknown rule inside expression: {rule:?}"),
    }
}

fn parse_expression(pairs: Pairs<Rule>) -> Result<Expression> {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::atom => parse_atom(primary),
            Rule::expr => parse_expression(primary.into_inner()),
            _ => unreachable!("Expr::parse expected atom, found {:?}", primary),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Operation::Add,
                Rule::subtract => Operation::Subtract,
                Rule::multiply => Operation::Multiply,
                Rule::divide => Operation::Divide,
                Rule::equals => Operation::Equals,
                Rule::not_equals => Operation::NotEquals,
                Rule::and => Operation::And,
                Rule::or => Operation::Or,
                Rule::greater => Operation::Greater,
                Rule::greater_eq => Operation::GreaterOrEquals,
                Rule::lesser => Operation::Lesser,
                Rule::lesser_eq => Operation::LesserOrEquals,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Ok(Expression::BinaryOperation {
                lhs: Box::new(lhs?),
                op,
                rhs: Box::new(rhs?),
            })
        })
        .parse(pairs)
}
