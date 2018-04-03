//! Coordinator of benchmark suite components.

use ::Sample;
use runner::Runner;


/// Uses a `Runner` to run a suite of benchmarks, recording a data sample for
/// each benchmark.
pub struct Bencher<R: Runner<S>, S> {
    name: Option<&'static str>,
    runner: R,
    samples: Vec<Sample<S>>,
}

impl<R: Runner<S>, S> Bencher<R, S> {
    /// Construct a new `Bencher` from a given `Runner`, which defines the
    /// strategy used to run each benchmark.
    pub fn new(runner: R) -> Self {
        Bencher {
            name: None,
            runner,
            samples: vec![],
        }
    }

    /// Run `target` according to the strategy of the configured `Runner`,
    /// recording a sample of data about `target`.
    ///
    /// This method is meant to be called indirectly via `Bencher::bench()`,
    /// inside a benchmark function. A benchmark function accepts a `Bencher` as
    /// an argument, possibly does some setup, and then calls `run()`.
    pub fn run<Target, Ret>(&mut self, mut target: Target)
        where Target: FnMut() -> Ret {

        let name = self.name.unwrap();
        let sample = self.runner.run(name, &mut target);
        self.samples.push(sample);
    }

    /// Execute the benchmark defined by `def`, named `name`.
    pub fn bench<Def>(&mut self, name: &'static str, def: &mut Def)
        where Def: FnMut(&mut Bencher<R, S>) {
        self.name = Some(name);
        def(self);
    }

    /// Borrow a slice of samples recorded by this `Bencher`.
    pub fn samples(&self) -> &Vec<Sample<S>> {
        &self.samples
    }
}
