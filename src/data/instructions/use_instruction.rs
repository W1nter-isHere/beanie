use crate::data::beanie_context::BeanieContext;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;
use crate::logger;

#[derive(Debug, Clone)]
pub struct UseInstruction {
    file_path: String
}

impl UseInstruction {
    pub fn new(file_path: String) -> UseInstruction {
        UseInstruction {
            file_path
        }
    }
}

impl Instruction for UseInstruction {
    fn execute(&self, context: &mut BeanieContext, parameters: &Vec<String>) {
    }

    fn add_argument(&mut self, name: String, expression: Expression) {
        logger::log_error("User instruction does not have any arguments")
    }
}