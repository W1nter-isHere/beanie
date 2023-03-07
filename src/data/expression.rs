use std::f64::consts;
use std::rc::Rc;
use mexprp::{Answer, Context, Expression, Num};
use mexprp::num::{ComplexFloat, ComplexRugRat};
use pest::iterators::Pair;
use rug::{Complex, Rational};
use crate::data::contexts::math_context::MathContext;
use crate::data::contexts::stripped_beanie_context::StrippedBeanieContext;
use crate::data::expression::data_type::DataType;
use crate::data::expression::expression_type::ExpressionType;
use crate::data::expression::funcs::{Abs, Acos, Asin, Atan, Atan2, Cos, Floor, Log, Max, Min, Nrt, Round, Sin, Sqrt, Tan};
use crate::interpreters::expression_parser::Rule;
use crate::utilities::file_utils::add_suffix;

pub mod data_type;
pub mod expression_type;

#[derive(Clone, Debug)]
pub enum BeanieExpression {
    Math(Pair<'static, Rule>, DataType),
    SimpleF64(f64),
    Boolean(bool),
    FilePath(String),
    DataType(DataType),
    String(String),
}

impl BeanieExpression {
    pub fn construct_math_context<N: Num + Clone + 'static>(&self, expr: &Pair<Rule>, bn_context: &StrippedBeanieContext) -> Context<N> {
        let mut math_context: Context<N> = Context::empty();
        let empty: Context<N> = Context::empty();
        
        for component in expr.clone().into_inner().flatten() {
            if component.as_rule() == Rule::variable_name {
                let name = component.as_str().trim().to_string();
                if math_context.vars.contains_key(&name) || math_context.funcs.contains_key(&name) {
                    continue;
                } 
                
                if bn_context.has_constant(&name) {
                    let constant_expression = bn_context.get_constant(&name).unwrap();
                    let result = constant_expression.0.evaluate::<N>(&MathContext::StrippedBeanie::<N>(bn_context.clone()));
                    let value = match result {
                        Answer::Single(answer) => answer,
                        Answer::Multiple(answers) => answers[constant_expression.1].clone(),
                    };
                    math_context.set_var(&name, value);
                } else if bn_context.has_function(&name) {
                    math_context.set_func(&name, bn_context.get_function(&name).unwrap());
                }
            } 
        }

        {
            math_context.set_var(add_suffix("pi", bn_context.context_suffix.as_str()).as_str(), N::from_f64(consts::PI, &empty).unwrap());
            math_context.set_var(add_suffix("e", bn_context.context_suffix.as_str()).as_str(), N::from_f64(consts::E, &empty).unwrap());
            math_context.set_var(add_suffix("i", bn_context.context_suffix.as_str()).as_str(), N::from_f64_complex((0.0, 1.0), &empty).unwrap());
            math_context.funcs.insert(add_suffix("sin", bn_context.context_suffix.as_str()), Rc::new(Sin));
            math_context.funcs.insert(add_suffix("cos", bn_context.context_suffix.as_str()), Rc::new(Cos));
            math_context.funcs.insert(add_suffix("max", bn_context.context_suffix.as_str()), Rc::new(Max));
            math_context.funcs.insert(add_suffix("min", bn_context.context_suffix.as_str()), Rc::new(Min));
            math_context.funcs.insert(add_suffix("sqrt", bn_context.context_suffix.as_str()), Rc::new(Sqrt));
            math_context.funcs.insert(add_suffix("nrt", bn_context.context_suffix.as_str()), Rc::new(Nrt));
            math_context.funcs.insert(add_suffix("tan", bn_context.context_suffix.as_str()), Rc::new(Tan));
            math_context.funcs.insert(add_suffix("abs", bn_context.context_suffix.as_str()), Rc::new(Abs));
            math_context.funcs.insert(add_suffix("asin", bn_context.context_suffix.as_str()), Rc::new(Asin));
            math_context.funcs.insert(add_suffix("acos", bn_context.context_suffix.as_str()), Rc::new(Acos));
            math_context.funcs.insert(add_suffix("atan", bn_context.context_suffix.as_str()), Rc::new(Atan));
            math_context.funcs.insert(add_suffix("atan2", bn_context.context_suffix.as_str()), Rc::new(Atan2));
            math_context.funcs.insert(add_suffix("floor", bn_context.context_suffix.as_str()), Rc::new(Floor));
            math_context.funcs.insert(add_suffix("round", bn_context.context_suffix.as_str()), Rc::new(Round));
            math_context.funcs.insert(add_suffix("log", bn_context.context_suffix.as_str()), Rc::new(Log));
        }   
        
        math_context
    }

    pub fn evaluate<N: Num + 'static>(&self, ctx: &MathContext<N>) -> Answer<N> {
        match self {
            BeanieExpression::Math(expr, _) => {
                let str = expr.as_str().trim();
                match ctx {
                    MathContext::StrippedBeanie(bn_context) => Expression::parse_ctx(str, self.construct_math_context::<N>(expr, &bn_context)).unwrap().eval().unwrap(),
                    MathContext::Math(math_context) => Expression::parse_ctx(str, math_context.clone()).unwrap().eval().unwrap(),
                }
            },
            BeanieExpression::SimpleF64(value) => N::from_f64(value.clone(), &Context::empty()).unwrap(),
            _ => unreachable!()
        }
    }
    
    pub fn evaluate_to_string(&self, context: &StrippedBeanieContext) -> String {
        match self {
            BeanieExpression::Math(expression_component, data_type) => {
                match data_type {
                    DataType::Decimal => self.evaluate::<f64>(&MathContext::StrippedBeanie::<f64>(context.clone())).to_string(),
                    DataType::ImaginaryDecimal => self.evaluate::<ComplexFloat>(&MathContext::StrippedBeanie::<ComplexFloat>(context.clone())).to_string(),
                    DataType::Complex => self.evaluate::<Complex>(&MathContext::StrippedBeanie::<Complex>(context.clone())).to_string(),
                    DataType::Rational => self.evaluate::<Rational>(&MathContext::StrippedBeanie::<Rational>(context.clone())).to_string(),
                    DataType::ComplexRational => self.evaluate::<ComplexRugRat>(&MathContext::StrippedBeanie::<ComplexRugRat>(context.clone())).to_string(),
                    _ => expression_component.as_str().to_string(),
                }
            } 
            BeanieExpression::Boolean(b) => b.to_string(),
            BeanieExpression::FilePath(file_path) => file_path.to_string(),
            BeanieExpression::DataType(data_type) => data_type.to_string(),
            BeanieExpression::String(str) => str.trim_matches('\'').to_string(),
            BeanieExpression::SimpleF64(value) => value.to_string(),
        }
    }
    
    pub fn get_math(&self) -> Option<Pair<'static, Rule>> {
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
            BeanieExpression::DataType(_) => ExpressionType::DataType,
            BeanieExpression::String(_) => ExpressionType::String,
            BeanieExpression::SimpleF64(_) => ExpressionType::MathExpression,
        }
    }
}

pub(in crate::data::expression) mod funcs {
    use std::cmp::Ordering;
    use mexprp::{Answer, Calculation, Context, Func, MathError, Num, Term};

    pub struct Sin;
    impl<N: Num + 'static> Func<N> for Sin {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::sin(a, ctx))
        }
    }

    pub struct Cos;
    impl<N: Num + 'static> Func<N> for Cos {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::cos(a, ctx))
        }
    }

    pub struct Max;
    impl<N: Num + 'static> Func<N> for Max {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.is_empty() {
                return Err(MathError::IncorrectArguments);
            }
            let mut extra = Vec::new();
            let mut max = match args[0].eval_ctx(ctx)? {
                Answer::Single(n) => n,
                Answer::Multiple(mut ns) => {
                    let one = ns.pop().unwrap();
                    extra = ns;
                    one
                }
            };

            // Try to evaluate the arguments
            let args: Vec<Answer<N>> = args.iter()
                .map(|term| term.eval_ctx(ctx))
                .collect::<Result<Vec<Answer<N>>, MathError>>()?;
            let mut new_args = Vec::new();
            // Push each answer of each argument to `new_args`
            for a in args {
                match a {
                    Answer::Single(n) => new_args.push(n),
                    Answer::Multiple(mut ns) => new_args.append(&mut ns),
                }
            }
            // For every argument as well as the extraneous solutions from the first one
            for arg in new_args[1..new_args.len()].iter().chain(extra.iter()) {
                if Num::tryord(arg, &max, ctx)? == Ordering::Greater {
                    max = arg.clone();
                }
            }
            Ok(Answer::Single(max))
        }
    }

    pub struct Min;
    impl<N: Num + 'static> Func<N> for Min {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.is_empty() {
                return Err(MathError::IncorrectArguments);
            }
            let mut extra = Vec::new();
            let mut min = match args[0].eval_ctx(ctx)? {
                Answer::Single(n) => n,
                Answer::Multiple(mut ns) => {
                    let one = ns.pop().unwrap();
                    extra = ns;
                    one
                }
            };

            // Try to evaluate the arguments
            let args: Vec<Answer<N>> = args.iter()
                .map(|term| term.eval_ctx(ctx))
                .collect::<Result<Vec<Answer<N>>, MathError>>()?;
            let mut new_args = Vec::new();
            // Push each answer of each argument to `new_args`
            for a in args {
                match a {
                    Answer::Single(n) => new_args.push(n),
                    Answer::Multiple(mut ns) => new_args.append(&mut ns),
                }
            }
            // For every argument as well as the extraneous solutions from the first one
            for arg in new_args[1..new_args.len()].iter().chain(extra.iter()) {
                if Num::tryord(arg, &min, ctx)? == Ordering::Less {
                    min = arg.clone();
                }
            }
            Ok(Answer::Single(min))
        }
    }

    pub struct Sqrt;
    impl<N: Num + 'static> Func<N> for Sqrt {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::sqrt(a, ctx))
        }
    }

    pub struct Nrt;
    impl<N: Num + 'static> Func<N> for Nrt {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 2 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;
            let b = args[1].eval_ctx(ctx)?;

            a.op(&b, |a, b| Num::nrt(a, b, ctx))
        }
    }

    pub struct Abs;
    impl<N: Num + 'static> Func<N> for Abs {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::abs(a, ctx))
        }
    }

    pub struct Tan;
    impl<N: Num + 'static> Func<N> for Tan {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::tan(a, ctx))
        }
    }

    pub struct Asin;
    impl<N: Num + 'static> Func<N> for Asin {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::asin(a, ctx))
        }
    }

    pub struct Acos;
    impl<N: Num + 'static> Func<N> for Acos {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::acos(a, ctx))
        }
    }

    pub struct Atan;
    impl<N: Num + 'static> Func<N> for Atan {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::atan(a, ctx))
        }
    }

    pub struct Atan2;
    impl<N: Num + 'static> Func<N> for Atan2 {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 2 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;
            let b = args[1].eval_ctx(ctx)?;

            a.op(&b, |a, b| Num::atan2(a, b, ctx))
        }
    }

    pub struct Floor;
    impl<N: Num + 'static> Func<N> for Floor {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::floor(a, ctx))
        }
    }

    pub struct Ceil;
    impl<N: Num + 'static> Func<N> for Ceil {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::ceil(a, ctx))
        }
    }

    pub struct Round;
    impl<N: Num + 'static> Func<N> for Round {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 1 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;

            a.unop(|a| Num::round(a, ctx))
        }
    }

    pub struct Log;
    impl<N: Num + 'static> Func<N> for Log {
        fn eval(&self, args: &[Term<N>], ctx: &Context<N>) -> Calculation<N> {
            if args.len() != 2 {
                return Err(MathError::IncorrectArguments);
            }

            let a = args[0].eval_ctx(ctx)?;
            let b = args[1].eval_ctx(ctx)?;

            a.op(&b, |a, b| Num::log(a, b, ctx))
        }
    }
}