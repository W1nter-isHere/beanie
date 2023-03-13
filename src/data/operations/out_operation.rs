use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;
use colored::Colorize;
use tree_sitter_beanie::data::instructions::out_operation_args::{OUT_TO_FILE, OUT_FILE, OUT_ARGUMENTS};
use crate::data::expression::BeanieExpression;
use crate::data::operations;
use crate::data::operations::Operation;
use crate::CLEANED_OUTPUT;
use crate::data::context::BeanieRuntimeContext;

#[derive(Debug, Clone)]
pub struct OutOperation {
    expression: BeanieExpression,
    arguments: HashMap<String, BeanieExpression>
}

impl OutOperation {
    pub fn new(expression: BeanieExpression) -> OutOperation {
        OutOperation {
            expression,
            arguments: HashMap::new(),
        }
    }
}

impl Operation for OutOperation {
    fn execute(&self, context: &mut BeanieRuntimeContext, _: &Vec<String>, _: &mut Vec<JoinHandle<()>>) {
        if self.arguments.contains_key(OUT_TO_FILE) && self.arguments[OUT_TO_FILE].evaluate::<f64>(context).unwrap_single().round() as i32 == true as i32 {
            let file_path = match self.arguments.contains_key(OUT_FILE) {
                true => self.arguments[OUT_FILE].evaluate_to_string(context),
                false => {
                    let path_buf = PathBuf::from(&context.beanie_file_path);
                    let output_file_name = path_buf.file_stem().unwrap().to_string_lossy().to_string() + "_out.txt";
                    
                    match path_buf.parent()
                    {
                        Some(path) => path.to_str().unwrap().to_string() + "/" + output_file_name.as_str(),
                        None => output_file_name,
                    }
                }
            };

            let mut open_option = OpenOptions::new();
            open_option.read(true);
            open_option.write(true);
            open_option.create(true);
            
            // if file already exist and we haven't yet cleaned it. We clean it.
            if Path::new(&file_path).is_file() && !CLEANED_OUTPUT.load(Ordering::Relaxed) {
                open_option.truncate(true);
                CLEANED_OUTPUT.store(true, Ordering::Relaxed);
            } else {
                open_option.append(true);
            } 
            
            let mut file = open_option
                .open(file_path)
                .unwrap_or_else(|err| panic!("{}. Error: {}", "Failed to open file".red(), err));
            
            file.write_all((self.expression.evaluate_to_string(context) + "\n").as_bytes())
                .unwrap_or_else(|err| panic!("{}. Error: {}", "Failed to write to output file".red(), err));
        }
    }

    fn add_argument(&mut self, name: String, expression: BeanieExpression) {
        operations::verify_argument("Out", &name, &expression, &OUT_ARGUMENTS, &mut self.arguments);
    }
}
