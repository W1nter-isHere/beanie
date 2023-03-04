use std::collections::HashMap;
use colored::Colorize;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use crate::data::beanie_context::BeanieContext;
use crate::data::data_type::DataType;
use crate::data::expression::Expression;
use crate::data::function::Function;
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
    let mut constants = HashMap::new();
    let mut functions = HashMap::new();
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
                            let expression = get_expression(&mut statement_components, &default_data_type);
                            
                            let instruction_obj: Box<dyn Instruction> = match instruction {
                                keywords::instructions::PRINT => Box::new(PrintInstruction::new(expression.clone())),
                                keywords::instructions::GRAPH => Box::new(GraphInstruction::new(expression.clone())),
                                keywords::instructions::USE => Box::new(UseInstruction::new(expression.clone())),
                                keywords::instructions::IN => Box::new(InInstruction::new(expression.clone())),
                                keywords::instructions::OUT => Box::new(OutInstruction::new(expression.clone())),
                                _ => {
                                    panic!("{}", format!("Instruction {} is not valid", instruction).red());
                                }
                            };
                            
                            instructions.push(instruction_obj);
                        },
                        Rule::operation_argument => {
                            match instructions.last_mut() {
                                Some(last_instruction) => {
                                    let argument_name = statement_components.next().unwrap().as_str();
                                    let argument_expression = get_expression(&mut statement_components, &default_data_type);
                                    last_instruction.add_argument(argument_name.to_string(), argument_expression)
                                }
                                None => {
                                    panic!("{}", "Instruction argument not following any instruction!".red());
                                }
                            }
                        },
                        Rule::function_declaration => {
                            let mut parameters: Vec<String> = Vec::new();
                            let function_name = statement_components.next().unwrap().as_str();
                            statement_components.next(); // skip the open parentheses
                            
                            // first parameter
                            parameters.push(statement_components.next().unwrap().as_str().to_string());

                            // as long as the next token is not closing parantheses, it means there are more parameters
                            while statement_components.next().filter(|token| token.as_rule() == Rule::close_parentheses).is_none() {
                                parameters.push(statement_components.next().unwrap().as_str().to_string());
                            }
                            
                            statement_components.next(); // skip the = sign
                            let expression = get_expression(&mut statement_components, &default_data_type);
                            
                            functions.insert(Function::new(function_name.to_string(), parameters), expression);
                        },
                        Rule::constant => {
                            
                        },
                       _ => unreachable!(),
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    };
    
    println!("{:?}", constants);
    println!("{:?}", functions);
    println!("{:?}", instructions);
    
    BeanieContext {
        constants,
        functions,
        instructions,
    }
}

fn get_expression(statement_components: &mut Pairs<Rule>, default_data_type: &DataType) -> Expression {
    let mut expression_components = statement_components.next().unwrap().into_inner();
    let math_expression = expression_components.next().unwrap();
    let datatype = get_as_datatype(&mut expression_components, &default_data_type);
    
    Expression::new(math_expression.as_str().to_string(), datatype)
}

fn get_as_datatype(expression: &mut Pairs<Rule>, default_data_type: &DataType) -> DataType {
    match expression.next() {
        Some(assumed_as_keyword) => {
            if assumed_as_keyword.as_rule() == Rule::as_keyword {
                expression.next().unwrap().as_str().parse::<DataType>().unwrap()
            } else {
                panic!("{}", "Math expression not followed by as keyword".red());
            }
        }
        None => default_data_type.clone(),
    }
}