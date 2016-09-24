use solver::Layer;

/// A solver stack.
pub struct Stack {
    /// The layers that comprise the stack.
    pub layers: Vec<Layer>,
    /// A human-readable string label.
    pub label: &str,
}
