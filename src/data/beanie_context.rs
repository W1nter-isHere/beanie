use std::collections::HashMap;
use crate::data::expression::Expression;
use crate::data::function::Function;
use crate::data::instructions::Instruction;

pub struct BeanieContext {
    pub constants: HashMap<Vec<String>, Expression>,
    pub functions: HashMap<Function, Expression>,
    pub instructions: Vec<Box<dyn Instruction>>,
}