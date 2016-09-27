use data::{Count, Game};
use solver::{Device, Layer, Outcome, Solution};

/// A solver stack.
pub trait Stack {
    fn layers(&self) -> Vec<Box<Layer>>;

    fn label(&self) -> &str;

    /// First, get a solution for the given game and depth. Second, put this
    /// solution back to the appropriate places in the stack.
    fn get_and_put(&self, game: &Game, depth: Count) -> Option<Solution> {
        let stack_get = self.get(game, depth);
        let opt_solution = stack_get.0;
        let devices = stack_get.1;
        opt_solution.map(|solution| self.put(game, solution, devices));
        opt_solution
    }

    #[allow(unused_variables)]
    fn put(&self, game: &Game, solution: Solution, devices: Vec<Box<Device>>) {
        unimplemented!()
    }

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
        let mut devices: Vec<Box<Device>> = Vec::new();
        for layer in self.layers().iter() {
            let device = layer.device();
            let opt_solution = if device.supports_read() {
                device.read(game)
            } else if device.supports_compute() {
                device.compute(game, depth)
            } else {
                None
            };
            match opt_solution {
                Some(solution) => {
                    if solution.is_deep_enough(depth) {
                        println!("[+] {}", layer.label());
                        return (Some(solution), devices);
                    } else {
                        println!("[-] {}", layer.label());
                        devices.push(device);
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
