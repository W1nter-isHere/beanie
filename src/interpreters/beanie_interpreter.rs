use crate::data::beanie_context::BeanieContext;
use crate::data::data_type::DataType;
use crate::interpreters::beanie_parser;

pub fn run(bn_file: String, parameters: &Vec<String>) {
    let mut context = beanie_parser::parse(bn_file);
    interpret(&mut context, &parameters);
}

fn interpret(context: &mut BeanieContext, parameters: &Vec<String>) {
    for instruction in context.instructions {
        instruction.execute(context, parameters);
    }
}
