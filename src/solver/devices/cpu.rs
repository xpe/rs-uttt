/// CPU Device.

use data::*;
use solver::*;

pub struct CPU {}

impl Device for CPU {
    fn read(&self, game: &Game, depth: Count) -> Option<Solution> {
        Some(game.solve(depth))
    }

    #[allow(unused_variables)]
    fn write(&self, game: &Game, solution: Solution) -> bool {
        false
    }

    fn is_writable(&self) -> bool {
        false
    }

    fn label(&self) -> &str {
        "CPU"
    }
}
