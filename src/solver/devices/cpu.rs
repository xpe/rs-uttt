/// CPU

use data::Game;
use solver::solution::Solution;
use solver::device::Device;

#[allow(dead_code)]
struct CPU {}

impl Device for CPU {
    #[allow(unused_variables)]
    fn get(&self, game: Game) -> Option<Solution> {
        None
    }

    #[allow(unused_variables)]
    fn put(&self, game: Game, solution: Solution) -> bool {
        false
    }

    fn has_put(&self) -> bool {
        false
    }

    fn label(&self) -> &str {
        "CPU"
    }
}
