use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

pub struct GraphInstruction {
    expression: Expression
}

impl GraphInstruction {
    pub fn new(expression: Expression) -> GraphInstruction {
        GraphInstruction {
            expression
        }
    }
}

impl Instruction for GraphInstruction {
    fn execute(&self) {
        todo!()
    }

    fn add_argument(&self, name: String, expression: Expression) {
        todo!()
    }
}