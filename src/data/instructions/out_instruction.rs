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
    fn execute(&self) {
        todo!()
    }

    fn add_argument(&mut self, name: String, expression: Expression) {
        todo!()
    }
}