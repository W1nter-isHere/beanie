use mexprp::num::{ComplexFloat, ComplexRugRat};
use rug::{Complex, Rational};
use crate::data::data_type::DataType;
use crate::data::expression::Expression;
use crate::data::instructions::Instruction;

pub struct PrintInstruction {
    expression: Expression
}

impl PrintInstruction {
    pub fn new(expression: Expression) -> PrintInstruction {
        PrintInstruction {
            expression
        }
    }
}

impl Instruction for PrintInstruction {
    fn execute(&self) {
        let result = match self.expression.get_type() {
            DataType::ComplexStruct => self.expression.evaluate::<Complex>().to_string(),
            DataType::ComplexFloat => self.expression.evaluate::<ComplexFloat>().to_string(),
            DataType::Rational => self.expression.evaluate::<Rational>().to_string(),
            DataType::ComplexRational => self.expression.evaluate::<ComplexRugRat>().to_string(),
            DataType::Irrational => self.expression.to_string(),
        };
        
        println!("{}", result)
    }

    fn add_argument(&self, name: String, expression: Expression) {
        todo!()
    }
}