use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use colored::Colorize;
use crate::data::beanie_context::BeanieContext;
use crate::data::data_type::DataType;
use crate::data::expression::Expression;
use crate::data::instructions;
use crate::data::instructions::Instruction;
use crate::keywords::booleans::TRUE;
use crate::CLEANED_OUTPUT;

const OUT_TO_FILE: &str = "output-to-file";
const OUT_FILE: &str = "output-file";

lazy_static! {
    static ref OUT_ARGUMENTS: HashMap<String, DataType> = hashmap!{
        String::from(OUT_TO_FILE) => DataType::Boolean,
        String::from(OUT_FILE) => DataType::FilePath,
    };
}

#[derive(Debug, Clone)]
pub struct OutInstruction {
    expression: Expression,
    arguments: HashMap<String, Expression>
}

impl OutInstruction {
    pub fn new(expression: Expression) -> OutInstruction {
        OutInstruction {
            expression,
            arguments: HashMap::new(),
        }
    }
}

impl Instruction for OutInstruction {
    fn execute(&self, context: &mut BeanieContext, parameters: &Vec<String>) {
        if self.arguments.contains_key(OUT_TO_FILE) && self.arguments[OUT_TO_FILE].evaluate_to_string(context) == TRUE { 
            
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

    fn add_argument(&mut self, name: String, expression: Expression) {
        instructions::verify_argument("Out", &name, &expression, &OUT_ARGUMENTS, &mut self.arguments);
    }
}