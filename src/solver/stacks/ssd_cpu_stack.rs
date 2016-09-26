/// SSD + CPU Stack.

use solver::*;

#[allow(non_camel_case_types)]
pub struct SSD_CPU_Stack {}

impl Stack for SSD_CPU_Stack {
    fn layers(&self) -> Vec<Box<Layer>> {
        vec![Box::new(SSD_Layer {}),
             Box::new(CPU_Layer {})]
    }

    fn label(&self) -> &str {
        "SSD + CPU"
    }
}
