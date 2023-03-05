extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;

mod data;
mod interpreters;
mod logger;
mod keywords;

use std::{env, fs};
use std::sync::atomic::AtomicBool;
use crate::data::data_type::DataType;
use crate::interpreters::{beanie_interpreter};

lazy_static! {
    pub static ref DEFAULT_DATA_TYPE: DataType = DataType::ComplexStruct;
    pub static ref CLEANED_OUTPUT: AtomicBool = AtomicBool::new(false);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // no file to run
    if args.len() <= 1 {
        logger::log_error("No file provided");
    }

    let file_path = &args[1];
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            logger::log_error(format!("Failed to read file {}", file_path).as_str());
            unreachable!()
        }
    };
    
    let parameters = &args[2..].to_vec();
    beanie_interpreter::run(file_path.clone(), file_content, &parameters);
}
