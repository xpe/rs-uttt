/// CPU Device.

use data::*;
use solver::*;

pub struct CPU {}

impl CPU {
    pub fn new() -> CPU {
        CPU {}
    }
}

impl Device for CPU {
    fn compute(&self, game: &Game, depth: Count, stack: &Stack)
               -> Option<Solution> {
        Some(game.solve(depth, stack))
    }

    #[allow(unused_variables)]
    fn read(&self, game: &Game) -> Option<Solution> {
        None
    }

    #[allow(unused_variables)]
    fn write(&self, game: &Game, solution: Solution) -> bool {
        false
    }

    fn supports_compute(&self) -> bool { true }
    fn supports_read(&self) -> bool { false }
    fn supports_write(&self) -> bool { false }

    fn label(&self) -> &str {
        "CPU"
    }
}
