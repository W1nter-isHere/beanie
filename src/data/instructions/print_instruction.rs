use std::fmt::Debug;
use crate::data::beanie_context::BeanieContext;
use crate::data::expression::BeanieExpression;
use crate::data::instructions::Instruction;
use crate::logger;

#[derive(Debug, Clone)]
pub struct PrintInstruction {
    expression: BeanieExpression
}

impl PrintInstruction {
    pub fn new(expression: BeanieExpression) -> PrintInstruction {
        PrintInstruction {
            expression
        }
    }
}

impl Instruction for PrintInstruction {
    fn execute(&self, context: &mut BeanieContext, _: &Vec<String>) {
        logger::log_info(self.expression.evaluate_to_string(context).as_str());
    }

    fn add_argument(&mut self, name: String, expression: BeanieExpression) {
        todo!()
    }
}