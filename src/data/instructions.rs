use std::collections::HashMap;
use std::fmt::Debug;
use crate::data::beanie_context::BeanieContext;
use crate::data::data_type::DataType;
use crate::data::expression::Expression;
use crate::logger;

pub mod print_instruction;
pub mod graph_instruction;
pub mod use_instruction;
pub mod in_instruction;
pub mod out_instruction;

pub trait Instruction: Debug {
    fn execute(&self, context: &mut BeanieContext, parameters: &Vec<String>);
    fn add_argument(&mut self, name: String, expression: Expression);
}

pub fn no_argument(instruction_name: &str) {
    logger::log_error(format!("There are no valid arguments to the instruction {}", instruction_name).as_str());
}

pub fn verify_argument(instruction_name: &str, name: &String, expression: &Expression, valid_arguments: &HashMap<String, DataType>, arguments: &mut HashMap<String, Expression>) {
    if valid_arguments.contains_key(name) {
        let expected_type = &valid_arguments[name];
        let got_type = &expression.get_type();
        
        if expected_type == got_type {
            arguments.insert(name.clone(), expression.clone());
            return;
        } else {
            logger::log_error(format!("Argument {} for instruction {} has the wrong type. Expected {}, got {}", name, instruction_name, expected_type, got_type).as_str())
        }
    }
    
    logger::log_error(format!("Argument {} is not valid for instruction {}", name, instruction_name).as_str());
}