use black_box::black_box;


pub struct Samples<'d> {
    pub name: &'static str,
    pub data: &'d [u64],
}

pub type TimerFn<T> = fn() -> T;
pub type DiffFn<T> = fn(&T, &T) -> u64;

pub struct Runner<T> {
    round_size: usize,
    timer: TimerFn<T>,
    diff: DiffFn<T>,
}

impl<'a, T> Runner<T> {
    pub fn new(round_size: usize, timer: TimerFn<T>, diff: DiffFn<T>) -> Self {
        Runner { round_size, timer, diff }
    }

    pub fn run<Target, Ret>(
        &mut self,
        target: &mut Target,
        samples: &mut [u64],
    ) where Target: FnMut() -> Ret {

        for i in 0..samples.len() {
            samples[i] = self.run_round(target);
        }
    }

    fn run_round<Target, Ret>(&mut self, target: &mut Target) -> u64
        where Target: FnMut() -> Ret {

        let start = (self.timer)();

        for _ in 0..self.round_size {
            black_box(target());
        }

        let end = (self.timer)();

        (self.diff)(&start, &end) / (self.round_size as u64)
    }
}
