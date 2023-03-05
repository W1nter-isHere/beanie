use mexprp::{Calculation, Context, Func, MathError, Num, Term};
use crate::data::expression::BeanieExpression;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub expression: BeanieExpression,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, expression: BeanieExpression) -> Function {
        Function {
            name,
            parameters,
            expression
        }
    }
}

impl<N: Num + 'static> Func<N> for Function {
    fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
        if args.len() != self.parameters.len() { return Err(MathError::IncorrectArguments) }
        
        let mut function_ctx = ctx.clone();
        for i in 0..args.len() {
            function_ctx.set_var(&self.parameters[i], args[i].eval_ctx(ctx).unwrap());
        }
        
        Ok(self.expression.evaluate_with_math_ctx(function_ctx))
    }
}