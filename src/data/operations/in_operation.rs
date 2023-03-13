use std::thread::JoinHandle;
use crate::DEFAULT_DATA_TYPE;
use crate::data::context::BeanieRuntimeContext;
use crate::data::expression::BeanieExpression;
use crate::data::operations::Operation;
use crate::data::operations;
use crate::utilities::logger;

#[derive(Debug, Clone)]
pub struct InOperation {
    input_name: String,
}

impl InOperation {
    pub fn new(input_name: String) -> InOperation {
        InOperation {
            input_name
        }
    }
}

impl Operation for InOperation {
    fn execute(&self, context: &mut BeanieRuntimeContext, parameters: &Vec<String>, _: &mut Vec<JoinHandle<()>>) {
        if let Some(index) = context.inputs.iter().position(|val| val == &self.input_name) {
            if index >= parameters.len() {
                logger::log_error("There are more required inputs than the number of inputs given");
                unreachable!()
            }
            
            unsafe {
                context.constants.insert(
                    vec![self.input_name.clone()],
                    BeanieExpression::Math(parameters[index].to_string(), DEFAULT_DATA_TYPE.clone()),
                );
            }
        } else { 
            logger::log_error(format!("Failed to find input {} within the file context", &self.input_name).as_str());
            unreachable!()
        }
    }

    fn add_argument(&mut self, _: String, _: BeanieExpression) {
        operations::no_argument("In");
    }
}
