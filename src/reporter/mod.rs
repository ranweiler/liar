//! Standard interface for reporting benchmark results.

pub mod line;

use ::Sample;


/// Can report benchmark results to an implementor-set output destination.
pub trait Reporter<S> {
    /// Given a slice of samples (from many benchmarks), report the results.
    fn report(&self, samples: &[Sample<S>]) -> Result<(), ()>;
}
