use std::collections::HashMap;
use crate::data::expression::BeanieExpression;
use crate::data::function::Function;
use crate::data::instructions::Instruction;

#[derive(Debug)]
pub struct BeanieContext {
    pub beanie_file_path: String,
    pub constants: HashMap<Vec<String>, BeanieExpression>,
    pub functions: HashMap<Function, BeanieExpression>,
    pub instructions: Vec<Box<dyn Instruction>>,
    pub inputs: Vec<String>,
    pub outputs: Vec<BeanieExpression>,
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