use std::fmt::{Debug, Display, Formatter};
use mexprp::{Answer, Calculation, Context, Func, MathError, Num, Term};
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
    
    fn evaluate_internal<N: Num + 'static>(&self, parameters: Vec<Answer<N>>, bn_ctx: &Option<StrippedBeanieContext>, ctx: &Context<N>) -> Calculation<N> {
        if parameters.len() != self.parameters.len() { return Err(MathError::IncorrectArguments) }

        let mut function_ctx = ctx.clone();
        for i in 0..parameters.len() {
            function_ctx.set_var(&self.parameters[i], parameters[i].clone());
        }

        if bn_ctx.is_some() {
            if let BeanieExpression::Math(expr, _) = &self.expression {
                let file_context = self.expression.construct_math_context::<N>(&expr, &bn_ctx.clone().unwrap());
                for (key, value) in file_context.funcs {
                    function_ctx.funcs.insert(key, value);
                }

                for (key, value) in file_context.vars {
                    function_ctx.vars.insert(key, value);
                }
            }
        }

        Ok(self.expression.evaluate(&MathContext::Math::<N>(function_ctx)))
    }
    
    pub fn evaluate(&self, parameters: Vec<f64>, bn_ctx: StrippedBeanieContext) -> f64 {
        let context: Context<f64> = Context::new();
        self.evaluate_internal::<f64>(parameters.iter().map(|s| Answer::Single(s.clone())).collect(), &Some(bn_ctx), &context).unwrap().unwrap_single()
    }
}

impl<N: Num + 'static> Func<N> for Function {
    fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
        self.evaluate_internal(args.iter().map(|arg| arg.eval_ctx(ctx).unwrap()).collect(), &self.external_context, ctx)
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