use crate::data::expression::Expression;

pub struct PrintInstruction {
    expression: Expression
}
impl Instruction for PrintInstruction {
    fn execute(&self) {
        self.expression
            .evaluate()
            .print_data();
    }
}

pub trait Instruction {
    fn execute(&self);
}