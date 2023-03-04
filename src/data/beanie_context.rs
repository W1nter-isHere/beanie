use std::collections::HashMap;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

pub struct BeanieContext {
    pub expressions: HashMap<Vec<String>, Expression>,
    pub instructions: Vec<Box<dyn Instruction>>,
}