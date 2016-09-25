/// CPU Stack.

use solver::*;

#[allow(non_camel_case_types)]
pub struct CPU_Stack {}

impl Stack for CPU_Stack {
    fn layers(&self) -> Vec<Box<Layer>> {
        vec![Box::new(CPU_Layer {})]
    }

    fn label(&self) -> &str {
        "CPU"
    }
}
