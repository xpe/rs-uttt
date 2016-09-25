use data::*;
use solver::*;

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

/// A solver layer.
pub trait Layer {
    fn device(&self) -> Box<Device>;

    fn policy(&self) -> Box<Policy>;

    fn label(&self) -> &str;
}

/// A solver device; e.g. RAM, SSD, HDD, or CPU.
pub trait Device {
    fn read(&self, game: &Game, depth: Count) -> Option<Solution>;

    fn write(&self, game: &Game, solution: Solution) -> bool;

    fn is_writable(&self) -> bool;

    fn label(&self) -> &str;
}

/// A solver policy.
pub trait Policy {
    fn label(&self) -> &str;
}
