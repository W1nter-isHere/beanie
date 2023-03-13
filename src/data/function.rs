use std::fmt::{Debug, Display, Formatter};
use mexprp::{Answer, Calculation, Context, Func, MathError, Num, Term};
use tree_sitter_beanie::data::context::FuncSignature;
use crate::data::context::BeanieRuntimeContext;
use crate::data::expression::BeanieExpression;

#[derive(Clone)]
pub struct Function {
    pub parameters: Vec<String>,
    pub expression: BeanieExpression,
    pub external_context: Option<BeanieRuntimeContext>,
}

impl Function {
    pub fn new(parameters: Vec<String>, expression: BeanieExpression) -> Function {
        Function {
            parameters,
            expression,
            external_context: None,
        }
    }
    
    fn evaluate_internal<N: Num + 'static>(&self, parameters: Vec<Answer<N>>, current_file_bn_context: Option<&BeanieRuntimeContext>, _: Option<&Context<N>>) -> Calculation<N> {
        if parameters.len() != self.parameters.len() { return Err(MathError::IncorrectArguments) }

        let mut parameters_ctx = Context::empty();

        for index in 0..parameters.len() {
            parameters_ctx.set_var(&self.parameters[index], parameters[index].clone());
        }
        
        if let Some(external_ctx) = &self.external_context {
            // if external ctx is present, meaning this function is external call to another bn
            // file. Therefore when evaluating this expression, we should not take in consideration
            // of the context in the currently evaluating bn file.

            // and the external file context itself
            return Ok(self.expression.evaluate_with_extra(Some(&external_ctx), vec![&parameters_ctx]));
        }
        
        // let mut ctxs = vec![&parameters_ctx];
        // if let Some(current_evaluation_context) = current_evaluation_context {
        //     ctxs.push(current_evaluation_context);
        // }
        // 
        // Ok(self.expression.evaluate_with_extra(current_file_bn_context, ctxs))

        Ok(self.expression.evaluate_with_extra(current_file_bn_context, vec![&parameters_ctx]))
    }
    
    pub fn evaluate(&self, parameters: Vec<f64>, bn_ctx: &BeanieRuntimeContext) -> f64 {
        self.evaluate_internal::<f64>(parameters.iter().map(|s| Answer::Single(*s)).collect(), Some(bn_ctx), None).unwrap().unwrap_single()
    }
}

impl<N: Num + 'static> Func<N> for Function {
    fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
        self.evaluate_internal(args.iter().map(|arg| arg.eval_ctx(ctx).unwrap()).collect(), None, Some(ctx))
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parameters: {:?}, Expression: {}", self.parameters, self.expression.get_math().unwrap().as_str())
    }
}

impl From<FuncSignature> for Function {
    fn from(value: FuncSignature) -> Self {
        Function::new(value.parameters, BeanieExpression::from(value.evaluation))
    }
}
