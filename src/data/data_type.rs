use crate::data::expression::Expression;

#[derive(Clone)]
pub enum DataType {
    NatNum(u32),
    Int(i32),
    Frac(i32, u32),
    Decimal(f32),
    IrratNum(Expression),
}

impl DataType {
    pub fn print_data(&self) {
        let text = match self {
            DataType::NatNum(num) => num.to_string(),
            DataType::Int(num) => num.to_string(),
            DataType::Frac(numerator, denominator) => format!("{}/{}", numerator, denominator),
            DataType::Decimal(num) => num.to_string(),
            DataType::IrratNum(expression) => expression.to_string(),
        };
        
        println!("{}", text);
    }
}