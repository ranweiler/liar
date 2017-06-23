#![cfg_attr(asm, feature(asm))]

mod black_box;
pub mod runner;

pub use self::black_box::black_box;

pub struct Bencher {
    name: Option<&'static str>,
    runner: runner::Runner,
    samples: Vec<runner::Samples>,
}

impl Bencher {
    pub fn new() -> Self {
        Bencher {
            name: None,
            runner: runner::Runner::new(),
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

    pub fn samples(&self) -> &Vec<runner::Samples> {
        &self.samples
    }
}

pub struct Reporter {}

impl Reporter {
    pub fn new() -> Self {
        Reporter {}
    }

    pub fn report(&self, samples: &Vec<runner::Samples>) {
        fn stats(s: &runner::Samples) -> (f64, f64, f64) {
            let total = s.data.iter().sum::<u64>() as f64;
            let n = s.data.len() as f64;
            let mean = total / n;
            let var = s.data
                .iter()
                .map(|x| ((*x as f64) - mean).powi(2))
                .sum::<f64>() / (n - 1.0);
            let ssd = var.sqrt();
            let ssd_perc = ssd / mean * 100.0;
            (mean, ssd, ssd_perc)
        }

        for s in samples {
            let (mean, ssd, ssd_perc) = stats(&s);
            println!("[{:}]:\t{:} ± {:.3} ({:.3}%)", s.name, mean, ssd, ssd_perc);
        }
    }
}
