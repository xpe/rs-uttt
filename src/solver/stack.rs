use data::*;
use solver::*;

pub struct Stack {
    pub devices: Vec<Device>,
}

/// A solver stack.
impl Stack {
    /// First, get one or more solutions for the given game and depth. Second,
    /// put the solution(s) back to the appropriate places in the stack.
    pub fn get_and_put(&self, game: &Game, depth: Count, stack: &Stack)
                       -> Vec<Solution> {
        let (solutions, devices) = self.get(game, depth, stack);
        // Only write solutions with depth greater than 0, since a depth == 0
        // solution can be looked up in a trivial amount of time. To write such
        // a trivial solution to a device would be wasteful.
        if depth > 0 {
            self.put(game, &solutions, devices);
        }
        solutions
    }

    /// Puts to some layer in the stack, guided by the supplied devices which
    /// have not-deep-enough copies of solutions. Such devices need to be
    /// updated, otherwise, they will continue to provide 'false positives' in
    /// the future.
    fn put(&self, game: &Game, solutions: &Vec<Solution>,
           devices: Vec<&Device>) -> bool {
        let devices_len: usize = devices.len();
        if devices_len == 0 {
            self.simple_put(game, solutions)
        } else {
            let mut count: usize = 0;
            for device in devices.iter() {
                if device.has_write {
                    if (device.write)(&device, game, solutions) {
                        count = count + 1
                    } else {
                        panic!("E3701");
                    }
                }
            }
            count == devices_len
        }
    }

    /// Iterates over the stack and attempt to put to each device in order.
    /// Returns true if successful; e.g. if one or more devices accepts the
    /// write.
    fn simple_put(&self, game: &Game, solutions: &Vec<Solution>) -> bool {
        let mut count: usize = 0;
        for device in self.devices.iter() {
            if device.has_write {
                if (device.write)(&device, game, solutions) {
                    count = count + 1
                } else {
                    panic!("E3702");
                }
            }
        }
        if count > 0 {
            true
        } else {
            println!("simple_put | devices.len() = {}", self.devices.len());
            panic!("E3703");
        }
    }

    /// Returns two things:
    ///
    /// 1. a vector of solutions for a given game and depth from the stack.
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
    fn get(&self, game: &Game, depth: Count, stack: &Stack)
           -> (Vec<Solution>, Vec<&Device>) {
        let mut devices: Vec<&Device> = Vec::new();
        for device in self.devices.iter() {
            let solutions = if device.has_read {
                (device.read)(&device, game)
            } else if device.has_compute {
                (device.compute)(game, depth, stack)
            } else {
                panic!("E3704");
            };
            let ss = solutions.iter()
                .filter(|sol| sol.is_deep_enough(depth))
                .cloned()
                .collect::<Vec<Solution>>();
            if ss.is_empty() {
                devices.push(device);
            } else {
                return (ss, devices);
            }
        }
        panic!("E3705");
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
