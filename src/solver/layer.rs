use solver::{Device, Policy};

/// A solver layer.
pub trait Layer {
    fn device(&self) -> Box<Device>;

    fn policy(&self) -> Box<Policy>;

    fn label(&self) -> &str;
}
