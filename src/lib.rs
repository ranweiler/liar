#![cfg_attr(asm, feature(asm))]

mod black_box;
pub mod reporter;
pub mod runner;

pub use self::black_box::black_box;

pub struct Bencher {
    name: Option<&'static str>,
    runner: runner::default::Runner,
    samples: Vec<runner::default::Samples>,
}

impl Bencher {
    pub fn new() -> Self {
        Bencher {
            name: None,
            runner: runner::default::Runner::new(),
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

    pub fn samples(&self) -> &Vec<runner::default::Samples> {
        &self.samples
    }
}
