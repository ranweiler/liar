use Sample;
use runner::{Runnable, Runner};

pub struct Bencher<R: Runner<S>, S> {
    name: Option<&'static str>,
    runner: R,
    samples: Vec<Sample<S>>,
}

impl<R: Runner<S>, S> Bencher<R, S> {
    pub fn new(runner: R) -> Self where {
        Bencher {
            name: None,
            runner,
            samples: vec![],
        }
    }

    pub fn run<Target, Ret>(&mut self, target: &mut Target)
    where
        Target: Runnable<Ret>,
    {
        let name = self.name.unwrap();
        let sample = self.runner.run(name, target);
        self.samples.push(sample);
    }

    pub fn bench<T, Ret>(&mut self, name: &'static str, mut target: T)
    where
        T: Runnable<Ret>,
    {
        self.name = Some(name);
        target.setup();
        self.run(&mut target);
        target.teardown();
    }

    pub fn samples(&self) -> &Vec<Sample<S>> {
        &self.samples
    }
}
