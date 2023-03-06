use std::collections::HashMap;
use crate::data::contexts::beanie_context::BeanieContext;
use crate::data::expression::BeanieExpression;
use crate::data::function::Function;

#[derive(Debug, Clone)]
pub struct StrippedBeanieContext {
    pub beanie_file_path: String,
    pub constants: HashMap<Vec<String>, BeanieExpression>,
    pub functions: Vec<Function>,
    pub inputs: Vec<String>,
    pub output: Option<BeanieExpression>,
}

impl StrippedBeanieContext {
    pub fn from(original: &BeanieContext) -> StrippedBeanieContext {
        StrippedBeanieContext {
            beanie_file_path: original.beanie_file_path.clone(),
            constants: original.constants.clone(),
            functions: original.functions.clone(),
            inputs: original.inputs.clone(),
            output: original.output.clone(),
        }
    }

    pub fn has_constant(&self, name: &str) -> bool {
        for key in self.constants.keys() {
            if key.iter().any(|s| s == name) {
                return true;
            }
        }
        false
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.functions.iter().any(|f| f.name == name)
    }

    pub fn get_constant(&self, name: &str) -> Option<(BeanieExpression, usize)> {
        for key in self.constants.keys() {
            let mut iterator = key.iter();
            if let Some(index) = iterator.position(|s| s == name) { return Some((self.constants[key].clone(), index)) }
        }

        None
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        for func in &self.functions {
            if func.name == name {
                return Some(func.clone());
            }
        }

        None
    }
}