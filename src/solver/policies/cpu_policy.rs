/// CPU Policy.

use solver::*;

#[allow(non_camel_case_types)]
pub struct CPU_Policy {}

impl Policy for CPU_Policy {
    fn label(&self) -> &str {
        "CPU"
    }
}
