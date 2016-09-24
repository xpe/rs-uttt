/// RAM

use data::Game;
use solver::solution::Solution;
use solver::device::Device;

#[allow(dead_code)]
struct RAM {}

impl Device for RAM {
    #[allow(unused_variables)]
    fn get(&self, game: Game) -> Option<Solution> {
        None
    }

    #[allow(unused_variables)]
    fn put(&self, game: Game, solution: Solution) -> bool {
        true
    }

    fn has_put(&self) -> bool {
        true
    }

    fn label(&self) -> &str {
        "RAM"
    }
}
