use crate::data::beanie_context::BeanieContext;
use crate::interpreters::beanie_parser;

pub fn run(bn_file_path: String, bn_file: String, parameters: &Vec<String>) {
    interpret(beanie_parser::parse(bn_file_path, bn_file), &parameters);
}

fn interpret(context: BeanieContext, parameters: &Vec<String>) {
    let mut stripped_context = BeanieContext::strip(&context);
    
    for instruction in context.instructions {
        instruction.execute(&mut stripped_context, parameters);
    }
    
    println!("{:#?}", stripped_context)
}