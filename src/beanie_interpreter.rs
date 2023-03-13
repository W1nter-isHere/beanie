use std::collections::HashMap;
use tree_sitter_beanie::data::context::BeanieParsingContext;
use tree_sitter_beanie::data::expression::data_type::DataType;
use tree_sitter_beanie::data::expression::instruction_expression::InstructionExpression;
use tree_sitter_beanie::data::instructions::types::OperationType;
use crate::data::context::BeanieRuntimeContext;
use crate::data::expression::BeanieExpression;
use crate::data::function::Function;
use crate::data::operations::graph_operation::GraphOperation;
use crate::data::operations::in_operation::InOperation;
use crate::data::operations::Operation;
use crate::data::operations::out_operation::OutOperation;
use crate::data::operations::print_operation::PrintOperation;
use crate::data::operations::use_operation::UseOperation;
use crate::utilities::logger;

pub fn parse(bn_file_path: String, bn_file: String, default_data_type: DataType) -> (BeanieRuntimeContext, Vec<Box<dyn Operation>>) {
    contextualize(bn_file_path, tree_sitter_beanie::parse(&bn_file, default_data_type, None).unwrap())
}

pub fn run(bn_file_path: String, bn_file: String, parameters: Vec<String>, default_data_type: DataType) {
    interpret(parse(bn_file_path, bn_file, default_data_type), parameters);
}

fn contextualize(bn_file_path: String, original: BeanieParsingContext) -> (BeanieRuntimeContext, Vec<Box<dyn Operation>>) {
    let mut context = BeanieRuntimeContext {
        beanie_file_path: bn_file_path,
        constants: HashMap::new(),
        functions: HashMap::new(),
        inputs: Vec::new(),
        output: None,
    };

    let mut instructions: Vec<Box<dyn Operation>> = Vec::new();

    for instruction in original.instructions {

        match instruction.operation_type {
            OperationType::Use => {
                if let InstructionExpression::FilePath(file_path) = instruction.expression {
                    instructions.push(Box::new(UseOperation::new(file_path)));
                } else {
                    logger::log_error("Use operation can be only followed by a file path expression");
                }
            }
            OperationType::In => {
                // todo: make in instruction only take in an identifier instead of a math expression
                if let InstructionExpression::Math(expr_sig) = instruction.expression {
                    instructions.push(Box::new(InOperation::new(expr_sig.evaluation.clone())));
                    context.inputs.push(expr_sig.evaluation);
                }
                else {
                    logger::log_error("In operation can be only followed by an identifier");
                }
            }
            OperationType::Out => {
                if let InstructionExpression::Math(expr_sig) = instruction.expression {
                    instructions.push(Box::new(OutOperation::new(BeanieExpression::from(expr_sig.clone()))));
                    context.output = Some(BeanieExpression::from(expr_sig));
                }
                else {
                    logger::log_error("Out operation can be only followed by a math expression");
                }
            }
            OperationType::Graph => {
                if let InstructionExpression::Math(expr_sig) = instruction.expression {
                    instructions.push(Box::new(GraphOperation::new(expr_sig.evaluation)))
                }
                else {
                    logger::log_error("Graph operation can be only followed by an identifier");
                }
            }
            OperationType::Print => {
                if let InstructionExpression::Math(expr_sig) = instruction.expression {
                    instructions.push(Box::new(PrintOperation::new(BeanieExpression::from(expr_sig))))
                }
                else {
                    logger::log_error("Out operation can be only followed by a math expression");
                } 
            }
        }

        for (name, argument) in instruction.arguments {
            if let Some(ins) = instructions.last_mut() { ins.add_argument(name, BeanieExpression::from(argument)) }
        }
    }

    for constant in original.constants {
        context.constants.insert(constant.0, BeanieExpression::from(constant.1));
    }
    for function in original.functions {
        context.functions.insert(function.0, Function::from(function.1));
    }

    (context, instructions)
}

fn interpret(context: (BeanieRuntimeContext, Vec<Box<dyn Operation>>), parameters: Vec<String>) {
    let mut threads_to_wait = Vec::new();
    let instructions = context.1;
    let mut context = context.0;

    for instruction in instructions {
        instruction.execute(&mut context, &parameters, &mut threads_to_wait);
    }

    for thread in threads_to_wait {
        thread.join().unwrap();
    }
}