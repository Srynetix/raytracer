use crate::RngWrapper;

/// Raytracer context.
/// Mostly used to centralize the RNG.
#[derive(Clone)]
pub struct Context {
    /// Random number generator.
    pub rng: RngWrapper,
}
