use solver::device::Device;
use solver::policy::Policy;

/// A solver layer.
pub struct Layer<D, P> where D: Device, P: Policy {
    pub device: D,
    pub policy: P,
}
