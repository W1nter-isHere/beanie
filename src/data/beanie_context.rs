use std::collections::HashMap;
use crate::data::expression::Expression;
use crate::data::instruction::Instruction;

pub struct BeanieContext {
    pub expressions: HashMap<String, Expression>,
    pub instructions: Vec<Box<dyn Instruction>>,
}

impl BeanieContext {
}