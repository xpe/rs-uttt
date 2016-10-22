/// SSD + CPU Stack.

use postgres::Connection;
use solver::*;

#[allow(non_camel_case_types)]
pub struct SSD_CPU_Stack {}

impl SSD_CPU_Stack {
    pub fn new<'c>(conn: &'c Connection) -> Stack<'c> {
        Stack {
            devices: vec![SSD::new(conn), CPU::new()],
        }
    }
}
