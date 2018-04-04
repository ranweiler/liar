//! Provides a benchmark target to a `Bencher`.

/// Provides a benchmark target to a `Bencher`.
pub trait Benchmark<Ret> {
    /// Get the name of the benchmark.
    fn name(&self) -> String;

    /// Set up context for the benchmark target.
    fn setup(&mut self);

    /// Target function to be measured.
    fn target(&mut self) -> Ret;

    /// Tear down context for the target.
    fn teardown(&mut self);
}
