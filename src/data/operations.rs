use std::collections::HashMap;
use std::fmt::Debug;
use std::thread::JoinHandle;
use tree_sitter_beanie::data::expression::expression_type::ExpressionType;
use crate::data::context::BeanieRuntimeContext;
use crate::data::expression::BeanieExpression;
use crate::utilities::logger;

pub mod print_operation;
pub mod graph_operation;
pub mod use_operation;
pub mod in_operation;
pub mod out_operation;

pub trait Operation: Debug {
    fn execute(&self, context: &mut BeanieRuntimeContext, parameters: &Vec<String>, threads_to_wait_for: &mut Vec<JoinHandle<()>>);
    fn add_argument(&mut self, name: String, expression: BeanieExpression);
}

pub fn no_argument(instruction_name: &str) {
    logger::log_error(format!("There are no valid arguments to the instruction {}", instruction_name).as_str());
}

pub fn verify_argument(instruction_name: &str, name: &String, expression: &BeanieExpression, valid_arguments: &HashMap<String, ExpressionType>, arguments: &mut HashMap<String, BeanieExpression>) {
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