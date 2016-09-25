use data::{Count, Game};
use solver::{Layer, Solution};

/// A solver stack.
pub trait Stack {
    fn layers(&self) -> Vec<Box<Layer>>;

    fn label(&self) -> &str;

    fn read(&self, game: &Game, depth: Count) -> Option<Solution> {
        for layer in self.layers().iter() {
            match layer.device().read(&game, depth) {
                Some(solution) => {
                    println!("[+] {}", layer.label());
                    return Some(solution)
                },
                None => {
                    println!("[ ] {}", layer.label());
                },
            }
        }
        None
    }
}
