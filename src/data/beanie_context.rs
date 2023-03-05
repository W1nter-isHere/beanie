use std::collections::HashMap;
use crate::data::expression::Expression;
use crate::data::function::Function;
use crate::data::instructions::Instruction;

pub struct BeanieContext {
    pub beanie_file_path: String,
    pub constants: HashMap<Vec<String>, Expression>,
    pub functions: HashMap<Function, Expression>,
    pub instructions: Vec<Box<dyn Instruction>>,
    pub inputs: Vec<String>,
    pub outputs: Vec<Expression>,
}

impl BeanieContext {
    pub fn strip(original: &BeanieContext) -> BeanieContext {
        BeanieContext {
            beanie_file_path: original.beanie_file_path.clone(),
            constants: original.constants.clone(),
            functions: original.functions.clone(),
            inputs: original.inputs.clone(),
            outputs: original.outputs.clone(),
            instructions: vec![],
        }
    }
}