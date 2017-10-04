use ::Sample;
use runner::Runner;


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

    pub fn run<Target, Ret>(&mut self, mut target: Target)
        where Target: FnMut() -> Ret {

        let name = self.name.unwrap();
        let sample = self.runner.run(name, &mut target);
        self.samples.push(sample);
    }

    pub fn bench<T>(&mut self, name: &'static str, target: &mut T)
        where T: FnMut(&mut Bencher<R, S>) {
        self.name = Some(name);
        target(self);
    }

    pub fn samples(&self) -> &Vec<Sample<S>> {
        &self.samples
    }
}
