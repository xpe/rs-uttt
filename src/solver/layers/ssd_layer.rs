/// SSD Layer.

use solver::*;

#[allow(non_camel_case_types)]
pub struct SSD_Layer {}

impl Layer for SSD_Layer {
    fn device(&self) -> Box<Device> {
        Box::new(SSD {})
    }

    fn policy(&self) -> Box<Policy> {
        Box::new(SSD_Policy {})
    }

    fn label(&self) -> &str {
        "SSD"
    }
}
