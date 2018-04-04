//! Strategies for running benchmarks.

use benchmark::Benchmark;

pub mod fixed;
pub mod linear;

use ::Sample;


/// Executes a benchmark target to record a sample.
pub trait Runner<S> {
    /// Runs and measures a target function according to the implementor's
    /// strategy, recording a sample of timing measurements.
    fn run<B, Ret>(&mut self, bench: &mut B) -> Sample<S>
    where
        B: Benchmark<Ret>;
}

/// A series of sequential executions of a benchmark target.
pub struct Round {
    /// Total time spent executing the target, in nanoseconds.
    pub ns: f64,

    /// Number of target executions in the round.
    pub size: usize,
}
