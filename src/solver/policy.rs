/// A solver policy.
pub trait Policy {
    /// Returns a human-presentable string.
    fn label(&self) -> &str;
}
