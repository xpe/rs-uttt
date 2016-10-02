/// SSD + CPU Stack.

use solver::*;

#[allow(non_camel_case_types)]
pub struct SSD_CPU_Stack {}

impl SSD_CPU_Stack {
    pub fn new() -> Stack {
        Stack {
            devices: vec![SSD::new(), CPU::new()],
        }
    }
}
