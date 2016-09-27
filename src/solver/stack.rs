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

    /// Puts to some layer in the stack, guided by the supplied devices which
    /// have not-deep-enough copies of solutions. Such devices need to be
    /// updated, otherwise, they will continue to provide 'false positives' in
    /// the future.
    fn put(&self, game: &Game, solution: Solution,
           devices: Vec<Box<Device>>) -> bool{
        if devices.is_empty() {
            self.simple_put(game, solution)
        } else {
            unimplemented!()
        }
    }

    /// Puts to some layer in the stack. Returns true if successful.
    fn simple_put(&self, game: &Game, solution: Solution) -> bool {
        for layer in self.layers().iter() {
            let device = layer.device();
            if device.supports_write() {
                if device.write(game, solution) {
                    println!("[>S] {}", layer.label()); // put succeeded
                    return true;
                } else {
                    println!("[>F] {}", layer.label()); // put failed
                }
            }
        }
        // Either (a) no device supports writes or (b) no write succeeded.
        false
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
                        return (Some(solution), devices);
                    } else {
                        devices.push(device);
                    }
                },
                None => {},
            }
        }
        (None, devices)
    }
}

impl Solution {
    /// Returns true if the provided solution is deep enough. A solution is
    /// deep enough if it is a: (a) win; (b) tie; or (c) unknown to a
    /// depth greater than or equal than the provided depth.
    fn is_deep_enough(self, depth: Count) -> bool {
        match self.outcome {
            Outcome::Unknown { turns: t } => t >= depth,
            _ => true,
        }
    }
}
