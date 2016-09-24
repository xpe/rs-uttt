use data::Game;
use solver::Solution;

/// A solver stack.
pub struct Stack<'a, D, P> where D: Device, P: Policy {
    /// The layers that comprise the stack.
    pub layers: Vec<Layer<D, P>>,
    /// A human-presentable string label.
    pub label: &'a str,
}

/// A solver layer.
pub struct Layer<D, P> where D: Device, P: Policy {
    /// The device in the layer.
    pub device: D,
    /// The policy
    pub policy: P,
}

/// A device always can read (get) and sometimes can write (put). If `put` is
/// supported, `has_put` must report `true`.
pub trait Device {
    /// Reads from the device.
    fn get(&self, game: Game) -> Option<Solution>;

    /// Writes to the device.
    fn put(&self, game: Game, solution: Solution) -> bool;

    /// Returns true if put is supported, false otherwise.
    fn has_put(&self) -> bool;

    /// Returns a human-presentable string label.
    fn label(&self) -> &str;
}

/// A solver policy.
pub trait Policy {
    /// Returns true when the game should be expired from the device.
    fn is_expired(&self, game: Game) -> bool;

    /// Returns a human-presentable string label.
    fn label(&self) -> &str;
}
