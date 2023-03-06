use std::fmt::{Debug, Display, Formatter};
use mexprp::{Calculation, Context, Func, MathError, Num, Term};
use crate::data::contexts::math_context::MathContext;
use crate::data::contexts::stripped_beanie_context::StrippedBeanieContext;
use crate::data::expression::BeanieExpression;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub expression: BeanieExpression,
    pub external_context: Option<StrippedBeanieContext>,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, expression: BeanieExpression) -> Function {
        Function {
            name,
            parameters,
            expression,
            external_context: None,
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

        if self.external_context.is_some() {
            if let BeanieExpression::Math(expr, _) = &self.expression {
                let file_context = self.expression.construct_math_context::<N>(&expr, &self.external_context.clone().unwrap());
                for (key, value) in file_context.funcs {
                    function_ctx.funcs.insert(key, value);
                }

                for (key, value) in file_context.vars {
                    function_ctx.vars.insert(key, value);
                }
            }
        }
        
        Ok(self.expression.evaluate(MathContext::Math::<N>(function_ctx)))
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Parameters: {:?}, Expression: {}", self.name, self.parameters, self.expression.get_math().unwrap().as_str())
    }
}