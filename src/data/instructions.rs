use std::fmt::Debug;
use crate::data::beanie_context::BeanieContext;
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
    
    fn no_argument(instruction_name: &str) {
        logger::log_error(format!("{} instruction does not have any arguments", instruction_name).as_str())
    }
}