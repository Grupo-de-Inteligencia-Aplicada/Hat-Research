use anyhow::bail;
use pest::Parser;
use pest_derive::Parser;

#[cfg(test)]
pub mod test;

#[derive(Parser)]
#[grammar = "grammars/hat.pest"]
pub struct HatParser;

fn main() -> anyhow::Result<()> {
    let src = include_str!("test/sample.hat");

    let file = match HatParser::parse(Rule::program, src) {
        Ok(file) => file,
        Err(e) => {
            panic!("{e:#?}");
        }
    };

    // println!("{file:#?}");
    // println!("OK!!!");

    for rule in file {
        println!("RULE: {:?}", rule.as_rule());
        match rule.as_rule() {
            Rule::handler_declaration => {
                let mut handler_decl = rule.into_inner();

                let handler_name = {
                    let name_rule = handler_decl.next().unwrap();

                    match name_rule.as_rule() {
                        Rule::string => {
                            let name_rule = name_rule.as_span().as_str();
                            name_rule[1..name_rule.len() - 1].to_owned()
                        },
                        Rule::ident => {
                            let name_rule = name_rule.as_span().as_str();
                            name_rule.to_owned()
                        },
                        _ => { bail!("unknown rule for handler name: {name_rule:?}") },
                    }
                };

                println!("RULE: {}", handler_name);
            }
            Rule::event_declaration => {
                // TODO:
            }
            _ => {}
        }
    }

    Ok(())
}
