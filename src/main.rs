extern crate pest;
#[macro_use]
extern crate pest_derive;

mod data;
mod interpreters;
mod logger;
mod keywords;

use std::{env, fs};
use crate::data::data_type::DataType;
use crate::interpreters::{beanie_parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // no file to run
    if args.len() <= 1 {
        logger::log_error("No file provided");
        return;
    }

    let file_path = &args[1];
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            logger::log_error(format!("Failed to read file {}", file_path).as_str());
            return;
        }
    };
    
    // beanie_interpreter::interpret(file_content);
       beanie_parser::parse(file_content, DataType::ComplexStruct);
}
