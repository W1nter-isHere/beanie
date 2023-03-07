use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use mexprp::Context;
use pgfplots::axis::{Axis, AxisKey};
use pgfplots::axis::plot::{Plot2D, PlotKey};
use pgfplots::{Engine, Picture};
use crate::data::contexts::math_context::MathContext;
use crate::data::contexts::stripped_beanie_context::StrippedBeanieContext;
use crate::data::expression::BeanieExpression;
use crate::data::instructions::Instruction;
use crate::utilities::logger;
use crate::data::expression::expression_type::ExpressionType;
use crate::data::instructions;

const MIN_X: &str = "min_x";
const MIN_Y: &str = "min_y";
const MAX_X: &str = "max_x";
const MAX_Y: &str = "max_y";
const STEP_X: &str = "step_x";
const STEP_Y: &str = "step_y";
const TITLE: &str = "title";
const LABEL_X: &str = "label_x";
const LABEL_Y: &str = "label_y";

lazy_static! {
    static ref GRAPH_ARGUMENTS: HashMap<String, ExpressionType> = hashmap!{
        String::from(MIN_X) => ExpressionType::MathExpression,
        String::from(MIN_Y) => ExpressionType::MathExpression,
        String::from(MAX_X) => ExpressionType::MathExpression,
        String::from(MAX_Y) => ExpressionType::MathExpression,
        String::from(STEP_X) => ExpressionType::MathExpression,
        String::from(STEP_Y) => ExpressionType::MathExpression,
        String::from(TITLE) => ExpressionType::String,
        String::from(LABEL_X) => ExpressionType::String,
        String::from(LABEL_Y) => ExpressionType::String,
    };
}

#[derive(Debug, Clone)]
pub struct GraphInstruction {
    function_name: String,
    arguments: HashMap<String, BeanieExpression>
}

impl GraphInstruction {
    pub fn new(function_name: String) -> GraphInstruction {
        GraphInstruction {
            function_name,
            arguments: hashmap! {
                String::from(MIN_X) => BeanieExpression::SimpleF64(-10f64),
                String::from(MIN_Y) => BeanieExpression::SimpleF64(-10f64),
                String::from(MAX_X) => BeanieExpression::SimpleF64(100f64),
                String::from(MAX_Y) => BeanieExpression::SimpleF64(100f64),
                String::from(STEP_X) => BeanieExpression::SimpleF64(10f64),
                String::from(STEP_Y) => BeanieExpression::SimpleF64(10f64),
            },
        }
    }
}

impl Instruction for GraphInstruction {
    fn execute(&self, context: &mut StrippedBeanieContext, _: &Vec<String>, threads_to_wait_for: &mut Vec<JoinHandle<()>>) {
        if !context.has_function(&self.function_name) {
            logger::log_error(format!("Can not graph function {} because it does not exist", self.function_name).as_str());
            unreachable!()
        }

        let function = context.get_function(&self.function_name).unwrap();
        let ctx = MathContext::Math(Context::empty());
        
        let min_x = self.arguments[MIN_X].evaluate::<f64>(&ctx).unwrap_single().round() as i32;
        let min_y = self.arguments[MIN_Y].evaluate::<f64>(&ctx).unwrap_single().round() as i32;
        let max_x = self.arguments[MAX_Y].evaluate::<f64>(&ctx).unwrap_single().round() as i32;
        let max_y = self.arguments[MAX_Y].evaluate::<f64>(&ctx).unwrap_single().round() as i32;
        let step_x = self.arguments[STEP_X].evaluate::<f64>(&ctx).unwrap_single();
        let step_y = self.arguments[STEP_Y].evaluate::<f64>(&ctx).unwrap_single();

        let mut plot = Plot2D::new();
        plot.coordinates = (min_x..max_x)
            .map(|i| (f64::from(i), function.evaluate(vec![f64::from(i)], context.clone())).into())
            .collect();
        
        let mut axis = Axis::new();
        
        if self.arguments.contains_key(TITLE) {
            axis.set_title(self.arguments[TITLE].evaluate_to_string(context));
        } 
            
        if self.arguments.contains_key(LABEL_X) { 
            axis.set_x_label(self.arguments[LABEL_X].evaluate_to_string(context));
        }

        if self.arguments.contains_key(LABEL_Y) {
            axis.set_y_label(self.arguments[LABEL_Y].evaluate_to_string(context));
        }

        axis.plots.push(plot);

        axis.add_key(AxisKey::Custom(format!("ymin={}, ymax={}", min_y, max_y)));
        axis.add_key(AxisKey::Custom(format!("xmin={}, xmax={}", min_x, max_x)));

        axis.add_key(AxisKey::Custom(format!("xtick distance={}", step_x)));
        axis.add_key(AxisKey::Custom(format!("ytick distance={}", step_y)));
        axis.add_key(AxisKey::Custom(String::from("axis lines=middle")));
        axis.add_key(AxisKey::Custom(String::from("xlabel near ticks")));
        axis.add_key(AxisKey::Custom(String::from("ylabel near ticks")));
        axis.add_key(AxisKey::Custom(String::from("scale only axis")));

        // we wanna spawn a separate thread to do this so it doesn't block the main thread
        threads_to_wait_for.push(thread::spawn(|| {
            Picture::from(axis).show_pdf(Engine::Tectonic).unwrap();
        }));
    }

    fn add_argument(&mut self, name: String, expression: BeanieExpression) {
        instructions::verify_argument("Graph", &name, &expression, &GRAPH_ARGUMENTS, &mut self.arguments);
    }
}