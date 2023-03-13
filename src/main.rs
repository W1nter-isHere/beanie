#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;

mod data;
mod beanie_interpreter;
mod utilities;

use std::env;
use std::sync::atomic::AtomicBool;
use tree_sitter_beanie::data::expression::data_type::DataType;
use crate::utilities::{file_utils, logger};

lazy_static! {
    pub static ref CLEANED_OUTPUT: AtomicBool = AtomicBool::new(false);
}

static mut DEFAULT_DATA_TYPE: DataType = DataType::Decimal;

fn main() {
    let args: Vec<String> = env::args().collect();

    // no file to run
    if args.len() <= 1 {
        logger::log_error("No file provided");
    }
    
    let file_path = &args[1];
    let parameters = match args.iter().position(|s| s.starts_with("--")) {
        Some(index) => { 
            let options = args[index..].to_vec();

            for option in options {
                let option = option[2..].to_string();

                match option.chars().into_iter().position(|c| c == '=') {
                    Some(index) => {
                        let option_data = option[index+1..].to_string();
                        let option = option[..index].to_string();
                        
                        match option.as_str() {
                            "default_data_type" => unsafe {
                                DEFAULT_DATA_TYPE = option_data.parse::<DataType>().unwrap();
                            },
                            _ => {
                                logger::log_info(format!("Unknown option {}", option).as_str())
                            }
                        }
                    }
                    None => {
                        todo!()
                    }
                }
            }
            
            args[2..index].to_vec()
        },
        None => args[2..].to_vec(),
    };

    unsafe {
        beanie_interpreter::run(
            file_path.clone(),
            file_utils::read_file(file_path),
            parameters,
            DEFAULT_DATA_TYPE.clone(),
        );
    }
}
