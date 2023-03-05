use crate::data::beanie_context::BeanieContext;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;
use crate::logger;

#[derive(Debug)]
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
    fn execute(&self, context: &mut BeanieContext, parameters: &Vec<String>)  {
        context.
    }

    fn add_argument(&mut self, _: String, _: Expression) {
        Instruction::no_argument("In");
    }
}