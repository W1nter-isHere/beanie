use std::path::PathBuf;
use std::thread::JoinHandle;
use crate::{DEFAULT_DATA_TYPE, beanie_interpreter};
use crate::data::context::BeanieRuntimeContext;
use crate::data::expression::BeanieExpression;
use crate::data::function::Function;
use crate::data::operations::Operation;
use crate::utilities::{file_utils, logger};

#[derive(Debug, Clone)]
pub struct UseOperation {
    file_path: String,
}

impl UseOperation {
    pub fn new(file_path: String) -> UseOperation {
        UseOperation {
            file_path
        }
    }
}

impl Operation for UseOperation {
    fn execute(&self, context: &mut BeanieRuntimeContext, _: &Vec<String>, _: &mut Vec<JoinHandle<()>>) {
        unsafe {
            let external_file = beanie_interpreter::parse(self.file_path.clone(), file_utils::read_file(&self.file_path), DEFAULT_DATA_TYPE.clone()).0; 
            if external_file.output.is_none() {
                logger::log_error(format!("Using {} but it has no output", self.file_path).as_str());
                unreachable!()
            }

            let name = PathBuf::from(&self.file_path).file_stem().unwrap().to_string_lossy().to_string();
            
            if context.has_function(&name) { return; }
            context.functions.insert(name, Function {
                parameters: external_file.inputs.clone(),
                expression: external_file.output.clone().unwrap(),
                external_context: Some(external_file),
            });
        }
    }

    fn add_argument(&mut self, _: String, _: BeanieExpression) {
        logger::log_error("User instruction does not have any arguments")
    }
}
