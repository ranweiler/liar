use runner;

pub struct Bencher {
    name: Option<&'static str>,
    runner: runner::fixed::FixedRunner,
    samples: Vec<runner::fixed::Samples>,
}

impl Bencher {
    pub fn new() -> Self {
        let runner = runner::fixed::FixedRunner::new();
        Self::from_runner(runner)
    }

    pub fn from_runner(runner: runner::fixed::FixedRunner) -> Self {
        Bencher {
            name: None,
            runner,
            samples: vec![],
        }
    }

    pub fn run<Target, Ret>(&mut self, mut target: Target)
        where Target: FnMut() -> Ret {

        let name = self.name.unwrap();
        let target_samples = self.runner.run(name, &mut target);
        self.samples.push(target_samples);
    }

    pub fn bench<T>(&mut self, name: &'static str, target: &mut T)
        where T: FnMut(&mut Bencher) {
        self.name = Some(name);
        target(self);
    }

    pub fn samples(&self) -> &Vec<runner::fixed::Samples> {
        &self.samples
    }
}
