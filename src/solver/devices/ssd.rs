/// SSD Device.

use data::*;
use solver::*;

pub struct SSD {}

impl Device for SSD {
    #[allow(unused_variables)]
    fn read(&self, game: &Game, depth: Count) -> Option<Solution> {
        None
    }

    #[allow(unused_variables)]
    fn write(&self, game: &Game, solution: Solution) -> bool {
        true
    }

    fn is_writable(&self) -> bool {
        true
    }

    fn label(&self) -> &str {
        "SSD"
    }
}
