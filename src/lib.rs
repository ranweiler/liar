#![cfg_attr(asm, feature(asm))]

mod black_box;

pub use self::black_box::black_box;

use std::time::{Instant, Duration};

pub struct Sample {
    name: &'static str,
    dur: Duration,
    count: u32,
}

pub struct Bencher {
    name: Option<&'static str>,
    samples: Vec<Sample>,
}

impl Bencher {
    pub fn new() -> Self {
        Bencher {
            name: None,
            samples: vec![],
        }
    }

    pub fn run<Target, Ret>(&mut self, mut target: Target)
        where Target: FnMut() -> Ret {
        let count = 100_000_u32;

        let now = Instant::now();
        for _ in 0..count {
            black_box(target());
        }
        let dur = now.elapsed();

        self.samples.push(Sample {
            name: self.name.unwrap(),
            dur,
            count,
        });
    }

    pub fn bench<T>(&mut self, name: &'static str, target: &mut T)
        where T: FnMut(&mut Bencher) {
        self.name = Some(name);
        target(self);
    }

    pub fn samples(&self) -> &Vec<Sample> {
        &self.samples
    }
}

pub struct Reporter {}

impl Reporter {
    pub fn new() -> Self {
        Reporter {}
    }

    pub fn report(&self, samples: &Vec<Sample>) {
        for s in samples {
            let ns = s.dur.as_secs() * 1_000_000_000 + (s.dur.subsec_nanos() as u64);
            let ns_per_iter = (ns as f64) / (s.count as f64);
            println!("{:}:\t{:?}", s.name, ns_per_iter);
        }
    }
}
