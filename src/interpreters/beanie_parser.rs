use std::collections::HashMap;
use colored::Colorize;
use pest::iterators::Pair;
use pest::Parser;
use crate::data::beanie_context::BeanieContext;
use crate::data::data_type::DataType;
use crate::data::expression::Expression;
use crate::data::instructions::graph_instruction::GraphInstruction;
use crate::data::instructions::in_instruction::InInstruction;
use crate::data::instructions::Instruction;
use crate::data::instructions::out_instruction::OutInstruction;
use crate::data::instructions::print_instruction::PrintInstruction;
use crate::data::instructions::use_instruction::UseInstruction;
use crate::keywords;

#[derive(Parser)]
#[grammar = "syntax/beanie_v0.1.pest"]
struct BeanieParser;

pub fn parse(bn_file: String, default_data_type: DataType) -> BeanieContext {
    let mut expressions = HashMap::new();
    let mut instructions = Vec::new();
    
    match BeanieParser::parse(Rule::file, bn_file.as_str()) {
        Ok(file) => {
            let f = file.clone().next().unwrap();

            // for each line in the file
            for line in f.into_inner().filter(|l: &Pair<Rule>| !l.as_str().is_empty()) {
                // turn it into the component that makes up the line
                for statement in line.into_inner() {
                    let mut statement_components = statement.clone().into_inner();
                    
                    // depending on which type we do something different
                    match statement.as_rule() {
                        Rule::operation => {
                            let instruction = statement_components.next().unwrap().as_str();
                            let expression = statement_components.next().unwrap().as_str();
                            let datatype = match statement_components.next() {
                                Some(assumed_as_keyword) => {
                                    if assumed_as_keyword.as_rule() == Rule::as_keyword {
                                        statement_components.next().unwrap().as_str().parse::<DataType>().unwrap()
                                    } else {
                                        panic!();
                                    }
                                }
                                None => default_data_type.clone(),
                            };
                            let expression_obj = Expression::new(expression.to_string(), datatype);

                            let instruction_obj: Box<dyn Instruction> = match instruction {
                                keywords::instructions::PRINT => Box::new(PrintInstruction::new(expression_obj.clone())),
                                keywords::instructions::GRAPH => Box::new(GraphInstruction::new(expression_obj.clone())),
                                keywords::instructions::USE => Box::new(UseInstruction::new(expression_obj.clone())),
                                keywords::instructions::IN => Box::new(InInstruction::new(expression_obj.clone())),
                                keywords::instructions::OUT => Box::new(OutInstruction::new(expression_obj.clone())),
                                _ => {
                                    panic!("{}", format!("Instruction {} is not valid", instruction).red());
                                }
                            };
                            
                            instructions.push(instruction_obj);
                        },
                        Rule::operation_argument => {
                            match instructions.last() {
                                Some(curr_instruction) => {
                                    let argument_name = statement_components.next().unwrap().as_str();
                                    let argument_expression = statement_components.next().unwrap().as_str();
                                    let datatype = match statement_components.next() {
                                        Some(assumed_as_keyword) => {
                                            if assumed_as_keyword.as_rule() == Rule::as_keyword {
                                                statement_components.next().unwrap().as_str().parse::<DataType>().unwrap()
                                            } else {
                                                panic!();
                                            }
                                        }
                                        None => default_data_type.clone(),
                                    };
                                    let expression_obj = Expression::new(argument_expression.to_string(), datatype);
                                    curr_instruction.add_argument(argument_name.to_string(), expression_obj)
                                }
                                None => {
                                    panic!("{}", "Instruction argument not following any instruction!".red());
                                }
                            }
                        },
                        Rule::function_declaration => {},
                        Rule::constant => {},
                       _ => unreachable!(),
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    };
    
    BeanieContext {
        expressions,
        instructions,
    }
}