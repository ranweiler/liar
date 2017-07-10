use runner::no_std::{DiffFn, Runner, Samples, TimerFn};


pub struct Bencher<'a, T> {
    name: Option<&'static str>,
    runner: Runner<T>,
    count: usize,
    samples: &'a mut [Option<Samples>],
}

impl<'a, T> Bencher<'a, T> {
    pub fn new(samples: &'a mut [Option<Samples>],
               timer: TimerFn<T>,
               diff: DiffFn<T>,
    ) -> Self {
        Bencher {
            name: None,
            runner: Runner::new(timer, diff),
            count: 0,
            samples,
        }
    }

    pub fn run<Target, Ret>(&mut self, mut target: Target)
        where Target: FnMut() -> Ret {

        let name = self.name.unwrap();
        let target_samples = self.runner.run(name, &mut target);

        self.samples[self.count] = Some(target_samples);
        self.count += 1;
    }

    pub fn bench<Target>(&mut self, name: &'static str, target: &mut Target)
        where Target: FnMut(&mut Bencher<T>) {
        self.name = Some(name);
        target(self);
    }

    pub fn samples(&self) -> &[Option<Samples>] {
        &self.samples[..self.count]
    }
}
