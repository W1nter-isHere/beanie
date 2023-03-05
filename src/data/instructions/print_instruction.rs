use std::fmt::Debug;
use crate::data::beanie_context::BeanieContext;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

#[derive(Debug, Clone)]
pub struct PrintInstruction {
    expression: Expression
}

impl PrintInstruction {
    pub fn new(expression: Expression) -> PrintInstruction {
        PrintInstruction {
            expression
        }
    }
}

impl Instruction for PrintInstruction {
    fn execute(&self, context: &mut BeanieContext, _: &Vec<String>) {
        println!("{}", self.expression.evaluate_to_string(context));
    }

    fn add_argument(&mut self, name: String, expression: Expression) {
        todo!()
    }
}