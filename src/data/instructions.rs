use crate::data::expression::Expression;

pub mod print_instruction;
pub mod graph_instruction;
pub mod use_instruction;
pub mod in_instruction;
pub mod out_instruction;

pub trait Instruction {
    fn execute(&self);
    fn add_argument(&self, name: String, expression: Expression);
}