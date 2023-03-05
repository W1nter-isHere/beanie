use mexprp::num::{ComplexFloat, ComplexRugRat};
use rug::{Complex, Rational};
use crate::data::beanie_context::BeanieContext;
use crate::data::data_type::DataType;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

#[derive(Debug)]
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
    fn execute(&self, context: &mut BeanieContext, parameters: &Vec<String>) {
    }

    fn add_argument(&mut self, name: String, expression: Expression) {
        Instruction::no_argument("Use");
    }
}