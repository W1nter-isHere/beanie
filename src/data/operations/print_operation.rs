use std::fmt::Debug;
use std::thread::JoinHandle;
use crate::data::context::BeanieRuntimeContext;
use crate::data::expression::BeanieExpression;
use crate::data::operations::Operation;
use crate::data::operations;
use crate::utilities::logger;

#[derive(Debug, Clone)]
pub struct PrintOperation {
    expression: BeanieExpression
}

impl PrintOperation {
    pub fn new(expression: BeanieExpression) -> PrintOperation {
        PrintOperation {
            expression
        }
    }
}

impl Operation for PrintOperation {
    fn execute(&self, context: &mut BeanieRuntimeContext, _: &Vec<String>, _: &mut Vec<JoinHandle<()>>) {
        logger::log_info(self.expression.evaluate_to_string(context).as_str());
    }

    fn add_argument(&mut self, _: String, _: BeanieExpression) {
        operations::no_argument("In");
    }
}
