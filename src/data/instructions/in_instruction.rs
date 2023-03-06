use crate::data::contexts::stripped_beanie_context::StrippedBeanieContext;
use crate::data::expression::BeanieExpression;
use crate::data::instructions::Instruction;
use crate::data::instructions;
use crate::interpreters::expression_parser;
use crate::utilities::logger;

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
    fn execute(&self, context: &mut StrippedBeanieContext, parameters: &Vec<String>) {
        if let Some(index) = context.inputs.iter().position(|val| val == &self.input_name) {
            if index >= parameters.len() {
                logger::log_error("There are more required inputs than the number of inputs given");
                unreachable!()
            }
            
            context.constants.insert(
                vec![self.input_name.clone()],
                expression_parser::parse(parameters[index].to_string(), "")
            );
        } else { 
            logger::log_error(format!("Failed to find input {} within the file context", &self.input_name).as_str());
            unreachable!()
        }
    }

    fn add_argument(&mut self, _: String, _: BeanieExpression) {
        instructions::no_argument("In");
    }
}