use std::path::PathBuf;
use std::thread::JoinHandle;
use crate::data::contexts::stripped_beanie_context::StrippedBeanieContext;
use crate::data::expression::BeanieExpression;
use crate::data::function::Function;
use crate::data::instructions::Instruction;
use crate::interpreters::beanie_parser;
use crate::utilities::{file_utils, logger};

#[derive(Debug, Clone)]
pub struct UseInstruction {
    file_path: String,
}

impl UseInstruction {
    pub fn new(file_path: String) -> UseInstruction {
        UseInstruction {
            file_path
        }
    }
}

impl Instruction for UseInstruction {
    fn execute(&self, context: &mut StrippedBeanieContext, _: &Vec<String>, _: &mut Vec<JoinHandle<()>>) {
        let external_file = beanie_parser::parse(self.file_path.clone(), file_utils::read_file(&self.file_path), file_utils::random_suffix().as_str()).strip();
        if external_file.output.is_none() {
            logger::log_error(format!("Using {} but it has no output", self.file_path).as_str());
            unreachable!()
        }

        let name = PathBuf::from(&self.file_path).file_stem().unwrap().to_string_lossy().to_string();
        
        if context.has_function(&name) { return; }
        context.functions.push(Function {
            name,
            parameters: external_file.inputs.clone(),
            expression: external_file.output.clone().unwrap(),
            external_context: Some(external_file),
        });
    }

    fn add_argument(&mut self, _: String, _: BeanieExpression) {
        logger::log_error("User instruction does not have any arguments")
    }
}