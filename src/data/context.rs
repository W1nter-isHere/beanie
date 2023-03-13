use std::collections::HashMap;
use crate::data::expression::BeanieExpression;
use crate::data::function::Function;

#[derive(Debug, Clone)]
pub struct BeanieRuntimeContext {
    pub beanie_file_path: String,
    pub constants: HashMap<Vec<String>, BeanieExpression>,
    pub functions: HashMap<String, Function>,
    pub inputs: Vec<String>,
    pub output: Option<BeanieExpression>,
}

impl BeanieRuntimeContext {
    pub fn has_constant(&self, name: &str) -> bool {
        for key in self.constants.keys() {
            if key.iter().any(|s| s == name) {
                return true;
            }
        }
        false
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn get_constant(&self, name: &str) -> Option<(BeanieExpression, usize)> {
        for key in self.constants.keys() {
            let mut iterator = key.iter();
            if let Some(index) = iterator.position(|s| s == name) { return Some((self.constants[key].clone(), index)) }
        }

        None
    }

    pub fn get_function(&self, name: &str) -> Option<Function> {
        if !self.has_function(name) { return None; }
        Some(self.functions[name].clone())
    }
}
