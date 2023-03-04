use mexprp::num::{ComplexFloat, ComplexRugRat};
use rug::{Complex, Rational};
use crate::data::data_type::DataType;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

pub struct UseInstruction {
    expression: Expression
}

impl UseInstruction {
    pub fn new(expression: Expression) -> UseInstruction {
        UseInstruction {
            expression
        }
    }
}

impl Instruction for UseInstruction {
    fn execute(&self) {
        todo!()
    }

    fn add_argument(&self, name: String, expression: Expression) {
        todo!()
    }
}