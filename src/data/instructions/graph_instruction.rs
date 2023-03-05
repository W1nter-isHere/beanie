use std::collections::HashMap;
use crate::data::beanie_context::BeanieContext;
use crate::data::expression::BeanieExpression;
use crate::data::instructions::Instruction;

#[derive(Debug, Clone)]
pub struct GraphInstruction {
    expression: BeanieExpression,
    arguments: HashMap<String, BeanieExpression>
}

impl GraphInstruction {
    pub fn new(expression: BeanieExpression) -> GraphInstruction {
        GraphInstruction {
            expression,
            arguments: HashMap::new(),
        }
    }
}

impl Instruction for GraphInstruction {
    fn execute(&self, context: &mut BeanieContext, parameters: &Vec<String>) {
        
    }

    fn add_argument(&mut self, name: String, expression: BeanieExpression) {
        self.arguments.insert(name, expression);
    }
}