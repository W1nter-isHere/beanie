use std::collections::HashMap;
use std::f64::consts;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use crate::data::contexts::beanie_context::BeanieContext;
use crate::data::expression::data_type::DataType;
use crate::data::expression::BeanieExpression;
use crate::data::function::Function;
use crate::data::instructions::graph_instruction::GraphInstruction;
use crate::data::instructions::in_instruction::InInstruction;
use crate::data::instructions::Instruction;
use crate::data::instructions::out_instruction::OutInstruction;
use crate::data::instructions::print_instruction::PrintInstruction;
use crate::data::instructions::use_instruction::UseInstruction;
use crate::utilities::{keywords, logger};
use crate::interpreters::expression_parser;
use crate::utilities::file_utils::add_suffix;
use crate::utilities::keywords::instructions;

#[derive(Parser)]
#[grammar = "syntax/beanie_v0.86.pest"]
struct BeanieParser;

pub fn parse(bn_file_path: String, bn_file: String, variable_suffix: &str) -> BeanieContext {
    let mut constants = HashMap::new();
    let mut functions = Vec::new();
    let mut instructions = Vec::new();
    let mut inputs = Vec::new();
    let mut output = None;
    
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
                            let expression = get_expression(&mut statement_components, variable_suffix);

                            let instruction_obj: Box<dyn Instruction> = match instruction {
                                keywords::instructions::PRINT => Box::new(PrintInstruction::new(expression)),
                                keywords::instructions::OUT => {
                                    if output.is_some() {
                                        logger::log_error("Can not have more than 1 output!");
                                        unreachable!()
                                    }
                                    
                                    output = Some(expression.clone());
                                    Box::new(OutInstruction::new(expression))
                                }
                                _ => {
                                    logger::log_error(format!("Instruction {} is not valid", instruction).as_str());
                                    unreachable!()
                                }
                            };

                            instructions.push(instruction_obj);
                        }
                        Rule::graph_operation => {
                            let name = add_suffix(statement_components.next().unwrap().as_str(), variable_suffix);
                            instructions.push(Box::new(GraphInstruction::new(name)));
                        }
                        Rule::in_operation => {
                            let name = add_suffix(statement_components.next().unwrap().as_str(), variable_suffix);
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
                                            Rule::file_path => BeanieExpression::FilePath(statement_components.next().unwrap().as_str().to_string()),
                                            Rule::boolean => BeanieExpression::Boolean(statement_components.next().unwrap().as_str().parse::<bool>().unwrap()),
                                            Rule::data_type => BeanieExpression::DataType(statement_components.next().unwrap().as_str().parse::<DataType>().unwrap()),
                                            Rule::expression => get_expression(&mut statement_components, variable_suffix),
                                            Rule::string => BeanieExpression::String(statement_components.next().unwrap().as_str().to_string()),
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
                            let function_name = add_suffix(statement_components.next().unwrap().as_str().trim(), variable_suffix);
                            if has_constant(&constants, function_name.as_str()) || has_function(&functions, function_name.as_str()) {
                                logger::log_error("Can not have 2 functions or constants with the same name");
                                unreachable!()
                            }
                            
                            statement_components.next(); // skip the open parentheses
                            
                            // if there are any parameters
                            if statement_components.peek().filter(|token| token.as_rule() == Rule::close_parentheses).is_none() {
                                loop {
                                    let parameter_name = statement_components.next().unwrap().as_str().trim().to_string();
                                    parameters.push(add_suffix(&parameter_name, variable_suffix));
                                    // as long as the next token is not closing parentheses, it means there are more parameters
                                    if statement_components.next().filter(|token| token.as_rule() == Rule::close_parentheses).is_some() { break; }
                                }
                            } else {  
                                statement_components.next(); // skip the )
                            }
                            
                            statement_components.next(); // skip the = sign
                            let expression = get_expression(&mut statement_components, variable_suffix);

                            functions.push(Function::new(function_name.to_string(), parameters, expression));
                        }
                        Rule::constant => {
                            let mut constant_names: Vec<String> = Vec::new();

                            loop {
                                let name = add_suffix(statement_components.next().unwrap().as_str().trim(), variable_suffix);
                                if has_constant(&constants, name.as_str()) || has_function(&functions, name.as_str()) {
                                    logger::log_error("Can not have 2 functions or constants with the same name");
                                    unreachable!()
                                }
                                constant_names.push(name);
                                if statement_components.next().filter(|token| token.as_rule() == Rule::comma).is_none() { break; }
                            }

                            constants.insert(constant_names, get_expression(&mut statement_components, variable_suffix));
                        }
                        _ => unreachable!()
                    }
                }
            }
        }
        Err(error) => { 
            logger::log_error(format!("\n{}", error).as_str());
            unreachable!()
        }
    };
    
    BeanieContext {
        beanie_file_path: bn_file_path,
        context_suffix: variable_suffix.to_string(),
        constants,
        functions,
        instructions,
        inputs,
        output,
    }
}

fn has_constant(constants: &HashMap<Vec<String>, BeanieExpression>, string: &str) -> bool {
    for key in constants.keys() {
        if key.iter().any(|s| s == string) {
            return true;
        }
    }
    false
}

fn has_function(functions: &Vec<Function>, string: &str) -> bool {
    functions.iter().any(|f| f.name == string)
}

fn get_expression(statement_components: &mut Pairs<Rule>, suffix: &str) -> BeanieExpression {
    expression_parser::parse(statement_components.next().unwrap().as_str().to_string(), suffix)
}