use std::collections::HashMap;
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
use crate::{keywords, logger};

#[derive(Parser)]
#[grammar = "syntax/grammar/beanie_v0.6.pest"]
struct BeanieParser;

pub fn parse(bn_file_path: String, bn_file: String) -> BeanieContext {
    let default_data_type = crate::DEFAULT_DATA_TYPE.clone();
    let mut constants = HashMap::new();
    let mut functions = HashMap::new();
    let mut instructions = Vec::new();
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

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
                            let instruction = statement_components.next().unwrap().as_str().trim();
                            let expression = get_expression(&mut statement_components, &default_data_type);

                            let instruction_obj: Box<dyn Instruction> = match instruction {
                                keywords::instructions::PRINT => Box::new(PrintInstruction::new(expression)),
                                keywords::instructions::GRAPH => Box::new(GraphInstruction::new(expression)),
                                keywords::instructions::OUT => {
                                    outputs.push(expression.clone());
                                    Box::new(OutInstruction::new(expression))
                                }
                                _ => {
                                    logger::log_error(format!("Instruction {} is not valid", instruction).as_str());
                                    unreachable!()
                                }
                            };

                            instructions.push(instruction_obj);
                        }
                        Rule::in_operation => {
                            let name = statement_components.next().unwrap().as_str().to_string();
                            inputs.push(name.clone());
                            instructions.push(Box::new(InInstruction::new(name)));
                        }
                        Rule::use_operation => {
                            instructions.push(Box::new(UseInstruction::new(statement_components.next().unwrap().as_str().to_string())));
                        }
                        Rule::operation_argument => {
                            match instructions.last_mut() {
                                Some(last_instruction) => {
                                    let argument_name = statement_components.next().unwrap().as_str().trim();
                                    let argument_expression = match statement_components.peek() {
                                        Some(argument_type) => match argument_type.as_rule() {
                                            Rule::file_path => Expression::new(statement_components.next().unwrap().as_str().to_string(), DataType::FilePath),
                                            Rule::boolean => Expression::new(statement_components.next().unwrap().as_str().to_string(), DataType::Boolean),
                                            Rule::data_type => Expression::new(statement_components.next().unwrap().as_str().to_string(), DataType::DataType),
                                            Rule::expression => get_expression(&mut statement_components, &default_data_type),
                                            _ => unreachable!(),
                                        }
                                        None => unreachable!(),
                                    };
                                    last_instruction.add_argument(argument_name.to_string(), argument_expression);
                                }
                                None => {
                                    logger::log_error("Instruction argument not following any instruction!");
                                }
                            }
                        }
                        Rule::function_declaration => {
                            let mut parameters: Vec<String> = Vec::new();
                            let function_name = statement_components.next().unwrap().as_str().trim();
                            statement_components.next(); // skip the open parentheses

                            // first parameter
                            parameters.push(statement_components.next().unwrap().as_str().trim().to_string());

                            // as long as the next token is not closing parentheses, it means there are more parameters
                            while statement_components.next().filter(|token| token.as_rule() == Rule::close_parentheses).is_none() {
                                parameters.push(statement_components.next().unwrap().as_str().trim().to_string());
                            }

                            statement_components.next(); // skip the = sign
                            let expression = get_expression(&mut statement_components, &default_data_type);

                            functions.insert(Function::new(function_name.to_string(), parameters), expression);
                        }
                        Rule::constant => {
                            let mut constant_names: Vec<String> = Vec::new();
                            constant_names.push(statement_components.next().unwrap().as_str().trim().to_string());

                            while statement_components.next().filter(|token| token.as_rule() == Rule::comma).is_some() {
                                constant_names.push(statement_components.next().unwrap().as_str().trim().to_string());
                            }

                            constants.insert(constant_names, get_expression(&mut statement_components, &default_data_type));
                        }
                        _ => unreachable!()
                    }
                }
            }
        }
        Err(error) => { 
            logger::log_error(format!("{}", error).as_str());
            unreachable!()
        }
    };

    println!("Constants: {:?}", constants);
    println!("Functions: {:?}", functions);
    println!("Instructions: {:?}", instructions);
    println!("Inputs: {:?}", inputs);
    println!("Outputs: {:?}", outputs);

    BeanieContext {
        beanie_file_path: bn_file_path,
        constants,
        functions,
        instructions,
        inputs,
        outputs,
    }
}

fn get_expression(statement_components: &mut Pairs<Rule>, default_data_type: &DataType) -> Expression {
    let mut expression_components = statement_components.next().unwrap().into_inner();
    let math_expression = expression_components.next().unwrap();
    let datatype = get_as_datatype(&mut expression_components, &default_data_type);
    Expression::new(math_expression, datatype)
}

fn get_as_datatype(expression: &mut Pairs<Rule>, default_data_type: &DataType) -> DataType {
    match expression.next() {
        Some(assumed_as_keyword) => {
            if assumed_as_keyword.as_rule() == Rule::as_keyword {
                expression.next().unwrap().as_str().trim().parse::<DataType>().unwrap()
            } else {
                logger::log_error("Math expression not followed by as keyword");
                unreachable!()
            }
        }
        None => default_data_type.clone(),
    }
}