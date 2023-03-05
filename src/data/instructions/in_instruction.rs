use pest::iterators::Pair;
use crate::data::beanie_context::BeanieContext;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;
use crate::{data, logger, main};
use crate::data::instructions;

#[derive(Debug, Clone)]
pub struct InInstruction {
    input_name: String,
}

impl InInstruction {
    pub fn new(input_name: String) -> InInstruction {
        InInstruction {
            input_name
        }
    }
}

impl Instruction for InInstruction {
    fn execute(&self, context: &mut BeanieContext, parameters: &Vec<String>) {
        if let Some(index) = context.inputs.iter().position(|val| val == &self.input_name) {
            context.constants.insert(
                vec![self.input_name.clone()],
                Expression::new(parameters[index].clone(), crate::DEFAULT_DATA_TYPE.clone()),
            );
        } else { 
            logger::log_error(format!("Failed to find input {} within the file context", &self.input_name).as_str());
            unreachable!()
        }
    }

    fn add_argument(&mut self, _: String, _: Expression) {
        instructions::no_argument("In");
    }
}