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
            has_compute: true,
            has_read: false,
            has_write: false,
            conn: None,
        }
    }

    fn compute(game: &Game, depth: Count, stack: &Stack) -> Option<Solution> {
        Some(game.solve(depth, stack))
    }

    #[allow(unused_variables)]
    fn read(device: &Device, game: &Game) -> Option<Solution> {
        panic!("Error 6399");
    }

    #[allow(unused_variables)]
    fn write(device: &Device, game: &Game, solution: Solution) -> bool {
        panic!("Error 9250");
    }

}
