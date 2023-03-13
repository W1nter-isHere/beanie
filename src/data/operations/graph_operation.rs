use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use pgfplots::axis::{Axis, AxisKey};
use pgfplots::axis::plot::Plot2D;
use pgfplots::{Engine, Picture};
use tree_sitter_beanie::data::instructions::graph_operation_args::{GRAPH_ARGUMENTS, MIN_X, MIN_Y, MAX_X, MAX_Y, STEP_X, STEP_Y, TITLE, LABEL_X, LABEL_Y};
use crate::data::context::BeanieRuntimeContext;
use crate::data::expression::BeanieExpression;
use crate::data::operations::Operation;
use crate::utilities::logger;
use crate::data::operations;

#[derive(Debug, Clone)]
pub struct GraphOperation {
    function_name: String,
    arguments: HashMap<String, BeanieExpression>
}

impl GraphOperation {
    pub fn new(function_name: String) -> GraphOperation {
        
        GraphOperation {
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

impl Operation for GraphOperation {
    fn execute(&self, context: &mut BeanieRuntimeContext, _: &Vec<String>, threads_to_wait_for: &mut Vec<JoinHandle<()>>) {
        if !context.has_function(&self.function_name) {
            logger::log_error(format!("Can not graph function {} because it does not exist", self.function_name).as_str());
            unreachable!()
        }

        let function = context.get_function(&self.function_name).unwrap();
        
        let min_x = self.arguments[MIN_X].evaluate::<f64>(context).unwrap_single().round() as i32;
        let min_y = self.arguments[MIN_Y].evaluate::<f64>(context).unwrap_single().round() as i32;
        let max_x = self.arguments[MAX_Y].evaluate::<f64>(context).unwrap_single().round() as i32;
        let max_y = self.arguments[MAX_Y].evaluate::<f64>(context).unwrap_single().round() as i32;
        let step_x = self.arguments[STEP_X].evaluate::<f64>(context).unwrap_single();
        let step_y = self.arguments[STEP_Y].evaluate::<f64>(context).unwrap_single();

        let mut plot = Plot2D::new();
        plot.coordinates = (min_x..max_x)
            .map(|i| (f64::from(i), f64::from(function.evaluate(vec![f64::from(i)], context))).into())
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
        operations::verify_argument("Graph", &name, &expression, &GRAPH_ARGUMENTS, &mut self.arguments);
    }
}
