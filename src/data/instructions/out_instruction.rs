use crate::data::beanie_context::BeanieContext;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

#[derive(Debug)]
pub struct OutInstruction {
    expression: Expression
}

impl OutInstruction {
    pub fn new(expression: Expression) -> OutInstruction {
        OutInstruction {
            expression
        }
    }
}

impl Instruction for OutInstruction {
    fn execute(&self, context: &mut BeanieContext, parameters: &Vec<String>) {
        todo!()
    }

    fn add_argument(&mut self, name: String, expression: Expression) {
        todo!()
    }
}