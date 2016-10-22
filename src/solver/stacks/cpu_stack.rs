/// CPU Stack.

use solver::*;

#[allow(non_camel_case_types)]
pub struct CPU_Stack {}

impl CPU_Stack {
    pub fn new<'c>() -> Stack<'c> {
        Stack {
            devices: vec![CPU::new()],
        }
    }
}
