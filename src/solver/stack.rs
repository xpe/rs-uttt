use data::{Count, Game};
use solver::{Device, Layer, Outcome, Solution};

/// A solver stack.
pub trait Stack {
    fn layers(&self) -> Vec<Box<Layer>>;

    fn label(&self) -> &str;

    /// Returns two things:
    ///
    /// 1. the first suitable solution for a given game and depth from the
    /// stack.
    ///
    /// 2. Returns a vector of devices (that were tried along the way) that
    /// had non-suitable solutions.
    ///
    /// This function starts with the highest part of the stack and works its
    /// way down. Delegates to either 'read' or 'compute' depending on the
    /// nature of the device.
    ///
    /// (Naming note: I chose the name 'get' to convey that it is more general
    /// than 'read' or 'compute'.)
    fn get(&self, game: &Game, depth: Count)
           -> (Option<Solution>, Vec<Box<Device>>) {
        let devices: Vec<Box<Device>> = vec![];
        for layer in self.layers().iter() {
            let device = layer.device();
            let opt_solution = if device.supports_read() {
                device.read(&game)
            } else if device.supports_compute() {
                device.compute(&game, depth)
            } else {
                None
            };
            match opt_solution {
                Some(solution) => {
                    if solution.is_deep_enough(depth) {
                        println!("[+] {}", layer.label());
                        return (Some(solution), devices)
                    } else {
                        println!("[-] {}", layer.label());
                        // TODO: Store this layer so that the algorithm can come
                        // back to it later and replace it with a more useful
                        // solution.
                    }
                },
                None => {
                    println!("[ ] {}", layer.label());
                },
            }
        }
        (None, devices)
    }
}

impl Solution {
    /// Returns true if the provided solution is deep enough. A solution is
    /// deep enough if it is a: (a) win; (b) tie; or (c) unknown to a
    /// depth equal to or greater than the provided depth.
    fn is_deep_enough(self, depth: Count) -> bool {
        match self.outcome {
            Outcome::Unknown { turns: t } => t < depth,
            _ => true,
        }
    }
}
