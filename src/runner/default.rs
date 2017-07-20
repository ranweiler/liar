use std::time::{Instant, Duration};

use black_box::black_box;


fn ns_from_dur(dur: &Duration) -> u64 {
    let ns_per_sec = 1_000_000_000_u64;
    (dur.as_secs() as u64) * ns_per_sec + (dur.subsec_nanos() as u64)
}

pub const DEFAULT_SAMPLE_SIZE: usize = 100;

pub struct Samples {
    pub name: &'static str,
    pub data: Vec<u64>,
}

pub struct Runner {
    sample_size: usize,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            sample_size: DEFAULT_SAMPLE_SIZE,
        }
    }

    pub fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Samples
        where Target: FnMut() -> Ret {

        let mut data = Vec::with_capacity(self.sample_size);

        for _ in 0..self.sample_size {
            data.push(self.run_loop(target));
        }

        Samples { name, data }
    }

    fn run_loop<Target, Ret>(&mut self, target: &mut Target) -> u64
        where Target: FnMut() -> Ret {

        let count = 10_000_u64;

        let now = Instant::now();
        for _ in 0..count {
            black_box(target());
        }
        let dur = now.elapsed();

        ns_from_dur(&dur) / count
    }
}
