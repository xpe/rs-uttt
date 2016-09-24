/// SSD

use data::Game;
use solver::{Device, Solution};

#[allow(dead_code)]
struct SSD {}

impl Device for SSD {
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
        "SSD"
    }
}
