//! Strategies for running benchmarks.

pub mod fixed;
pub mod linear;

use ::Sample;


/// Executes a benchmark target to record a sample.
pub trait Runner<S> {
    /// Runs and measures a target function according to the implementor's
    /// strategy, recording a sample of timing measurements.
    fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<S>
        where Target: FnMut() -> Ret;
}

/// A series of sequential executions of a benchmark target.
pub struct Round {
    /// Total time spent executing the target, in nanoseconds.
    pub ns: f64,

    /// Number of target executions in the round.
    pub size: usize,
}
