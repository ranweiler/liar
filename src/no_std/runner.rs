use black_box::black_box;


pub struct Samples<'d> {
    pub name: &'static str,
    pub data: &'d [u64],
}

pub type TimerFn<T> = fn() -> T;
pub type DiffFn<T> = fn(&T, &T) -> u64;

pub struct Runner<T> {
    rounds: usize,
    timer: TimerFn<T>,
    diff: DiffFn<T>,
}

impl<'a, T> Runner<T> {
    pub fn new(rounds: usize, timer: TimerFn<T>, diff: DiffFn<T>) -> Self {
        Runner { rounds, timer, diff }
    }

    pub fn run<Target, Ret>(
        &mut self,
        target: &mut Target,
        samples: &mut [u64],
    ) where Target: FnMut() -> Ret {

        for i in 0..samples.len() {
            samples[i] = self.run_loop(target);
        }
    }

    fn run_loop<Target, Ret>(&mut self, target: &mut Target) -> u64
        where Target: FnMut() -> Ret {

        let start = (self.timer)();

        for _ in 0..self.rounds {
            black_box(target());
        }

        let end = (self.timer)();

        (self.diff)(&start, &end) / (self.rounds as u64)
    }
}
