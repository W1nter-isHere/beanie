use std::collections::HashMap;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

#[derive(Debug)]
pub struct GraphInstruction {
    expression: Expression,
    arguments: HashMap<String, Expression>
}

impl GraphInstruction {
    pub fn new(expression: Expression) -> GraphInstruction {
        GraphInstruction {
            expression,
            arguments: HashMap::new(),
        }
    }
}

impl Instruction for GraphInstruction {
    fn execute(&self) {
        todo!()
    }

    fn add_argument(&mut self, name: String, expression: Expression) {
        self.arguments.insert(name, expression);
    }
}