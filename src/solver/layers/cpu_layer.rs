/// CPU Layer.

use solver::*;

#[allow(non_camel_case_types)]
pub struct CPU_Layer {}

impl Layer for CPU_Layer {
    fn device(&self) -> Box<Device> {
        Box::new(CPU {})
    }

    fn policy(&self) -> Box<Policy> {
        Box::new(CPU_Policy {})
    }

    fn label(&self) -> &str {
        "CPU"
    }
}
