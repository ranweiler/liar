//! Coordinator of benchmark suite components.

use ::Sample;
use benchmark::Benchmark;
use runner::Runner;


/// Uses a `Runner` to run a suite of benchmarks, recording a data sample for
/// each benchmark.
pub struct Bencher<R: Runner<S>, S> {
    runner: R,
    samples: Vec<Sample<S>>,
}

impl<R: Runner<S>, S> Bencher<R, S> {
    /// Construct a new `Bencher` from a given `Runner`, which defines the
    /// strategy used to run each benchmark.
    pub fn new(runner: R) -> Self {
        Bencher {
            runner,
            samples: vec![],
        }
    }

    /// Execute the benchmark `bench`.
    pub fn run<B, Ret>(&mut self, bench: &mut B)
    where B: Benchmark<Ret>,
    {
        bench.setup();
        let sample = self.runner.run(bench);
        self.samples.push(sample);
        bench.teardown();
    }

    /// Borrow a slice of samples recorded by this `Bencher`.
    pub fn samples(&self) -> &[Sample<S>] {
        &self.samples
    }
}
