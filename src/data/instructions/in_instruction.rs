use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

pub struct InInstruction {
    expression: Expression
}

impl InInstruction {
    pub fn new(expression: Expression) -> InInstruction {
        InInstruction {
            expression
        }
    }
}

impl Instruction for InInstruction {
    fn execute(&self) {
        todo!()
    }

    fn add_argument(&self, name: String, expression: Expression) {
        todo!()
    }
}