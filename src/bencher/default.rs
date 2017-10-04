use runner::fixed;

pub struct Bencher {
    name: Option<&'static str>,
    runner: fixed::FixedRunner,
    samples: Vec<fixed::Samples>,
}

impl Bencher {
    pub fn new() -> Self {
        let runner = fixed::FixedRunner::new(
            fixed::DEFAULT_ROUND_SIZE,
            fixed::DEFAULT_SAMPLE_SIZE,
        );

        Self::from_runner(runner)
    }

    pub fn from_runner(runner: fixed::FixedRunner) -> Self {
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

    pub fn samples(&self) -> &Vec<fixed::Samples> {
        &self.samples
    }
}
