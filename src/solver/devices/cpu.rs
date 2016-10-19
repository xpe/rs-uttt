/// CPU Device.

use data::*;
use solver::*;

pub struct CPU {}

impl CPU {
    pub fn new() -> Device {
        Device {
            compute: CPU::compute,
            read: CPU::read,
            write: CPU::write,
            flush: CPU::flush,
            has_compute: true,
            has_read: false,
            has_write: false,
            has_flush: false,
            pool: None,
            cache_1: None,
            cache_2: None,
        }
    }

    fn compute(game: &Game, depth: Count, stack: &Stack) -> Vec<Solution> {
        game.solve(depth, stack)
    }

    #[allow(unused_variables)]
    fn read(device: &Device, game: &Game) -> Vec<Solution> {
        panic!("E6301");
    }

    #[allow(unused_variables)]
    fn write(device: &Device, game: &Game, sols: &Vec<Solution>) -> bool {
        panic!("E6302");
    }

    #[allow(unused_variables)]
    fn flush(device: &Device) -> (bool, usize) {
        panic!("E6303");
    }
}
