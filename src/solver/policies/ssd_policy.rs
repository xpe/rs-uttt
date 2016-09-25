/// CPU Policy.

use solver::*;

#[allow(non_camel_case_types)]
pub struct SSD_Policy {}

impl Policy for SSD_Policy {
    fn label(&self) -> &str {
        "SSD"
    }
}
