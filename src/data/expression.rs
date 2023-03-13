use mexprp::{Answer, Context, Num, Expression};
use mexprp::num::{ComplexFloat, ComplexRugRat};
use regex::Regex;
use rug::{Complex, Rational};
use tree_sitter_beanie::data::context::ExpressionSignature;
use tree_sitter_beanie::data::expression::data_type::DataType;
use tree_sitter_beanie::data::expression::expression_type::ExpressionType;
use tree_sitter_beanie::data::expression::instruction_expression::InstructionExpression;
use crate::data::context::BeanieRuntimeContext;

#[derive(Clone, Debug)]
pub enum BeanieExpression {
    Math(String, DataType),
    SimpleF64(f64),
    Boolean(bool),
    FilePath(String),
    String(String),
}

impl BeanieExpression {
    fn build_context<N: Num + 'static>(str: &str, ctx: Option<&BeanieRuntimeContext>) -> Context<N>{
        if let Some(ctx) = ctx {
            let pattern = Regex::new(r"[a-zA-Z][a-zA-Z0-9_]*").unwrap();
            let mut context = Context::new();

            for ident in pattern.find_iter(str) {
                let name = ident.as_str();
                
                // todo
                if ctx.has_constant(name) {
                    let constant = ctx.get_constant(name).unwrap();
                    let expression = constant.0;
                    let index = constant.1;

                    let value = match expression.evaluate::<N>(ctx) {
                        Answer::Single(ans) => ans,
                        Answer::Multiple(ans) => ans[index].clone(),
                    };

                    context.set_var(name, value);

                } else if ctx.has_function(name) {
                    let function = ctx.get_function(name).unwrap();
                    context.set_func(name, function);
                }
            }
            
            return context;
        }

        Context::empty()
    }
    
    fn evaluate_with_math_ctx<N: Num + 'static>(expr: &str, ctx: &Context<N>, extra_ctxs: Vec<&Context<N>>) -> Answer<N> {
        if !extra_ctxs.is_empty() {

            let mut final_ctx = ctx.clone();

            for extra_ctx in extra_ctxs {
                for (name, func) in &extra_ctx.funcs {
                    final_ctx.funcs.insert(name.clone(), func.clone());
                }

                for (name, var) in &extra_ctx.vars {
                    final_ctx.vars.insert(name.clone(), var.clone());
                }
            }

            return Expression::parse_ctx(expr, final_ctx).unwrap().eval().unwrap();
        }
        
        Expression::parse_ctx(expr, ctx.clone()).unwrap().eval().unwrap()
    }
    
    pub fn evaluate_with_extra<N: Num + 'static>(&self, ctx: Option<&BeanieRuntimeContext>, extra_ctx: Vec<&Context<N>>) -> Answer<N> {
        match self {
            BeanieExpression::Math(expr, _) => {
                let str = expr.as_str().trim();
                BeanieExpression::evaluate_with_math_ctx(str, &BeanieExpression::build_context(str, ctx), extra_ctx)
            },
            BeanieExpression::SimpleF64(value) => N::from_f64(value.clone(), &Context::empty()).unwrap(),
            BeanieExpression::Boolean(b) => N::from_f64(if b.clone() { 1f64 } else { 0f64 }, &Context::empty()).unwrap(),
            _ => unreachable!()
        }
    }

    pub fn evaluate<N: Num + 'static>(&self, ctx: &BeanieRuntimeContext) -> Answer<N> {
        self.evaluate_with_extra(Some(ctx), Vec::new())
    }
   
    pub fn evaluate_to_string(&self, context: &BeanieRuntimeContext) -> String {
        match self {
            BeanieExpression::Math(expression_component, data_type) => {
                match data_type {
                    DataType::Decimal => self.evaluate::<f64>(context).to_string(),
                    DataType::ImaginaryDecimal => self.evaluate::<ComplexFloat>(context).to_string(),
                    DataType::Complex => self.evaluate::<Complex>(context).to_string(),
                    DataType::Rational => self.evaluate::<Rational>(context).to_string(),
                    DataType::ComplexRational => self.evaluate::<ComplexRugRat>(context).to_string(),
                    _ => expression_component.as_str().to_string(),
                }
            } 
            BeanieExpression::Boolean(b) => b.to_string(),
            BeanieExpression::FilePath(file_path) => file_path.to_string(),
            BeanieExpression::String(str) => str.trim_matches('\'').to_string(),
            BeanieExpression::SimpleF64(value) => value.to_string(),
        }
    }
    
    pub fn get_math(&self) -> Option<String> {
        match self {
            BeanieExpression::Math(math, _) => Some(math.clone()),
            _ => None,
        }
    }
    
    pub fn get_type(&self) -> ExpressionType {
        match self {
            BeanieExpression::Math(_, _) => ExpressionType::MathExpression,
            BeanieExpression::Boolean(_) => ExpressionType::Boolean,
            BeanieExpression::FilePath(_) => ExpressionType::FilePath,
            BeanieExpression::String(_) => ExpressionType::String,
            BeanieExpression::SimpleF64(_) => ExpressionType::MathExpression,
        }
    }
}

impl From<ExpressionSignature> for BeanieExpression {
    fn from(value: ExpressionSignature) -> Self {
        BeanieExpression::Math(value.evaluation.clone(), value.data_type.clone())
    }
}

impl From<InstructionExpression> for BeanieExpression {
    fn from(value: InstructionExpression) -> Self {
        match value {
            InstructionExpression::Math(math) => BeanieExpression::from(math),
            InstructionExpression::FilePath(fp) => BeanieExpression::FilePath(fp),
            InstructionExpression::Boolean(val) => BeanieExpression::Boolean(val),
            InstructionExpression::String(str) => BeanieExpression::String(str),
        }
    }
}
