use black_box::black_box;


pub const SAMPLE_SIZE: usize = 100;

pub struct Samples {
    pub name: &'static str,
    pub data: [u64; SAMPLE_SIZE],
}

pub type TimerFn<T> = fn() -> T;
pub type DiffFn<T> = fn(&T, &T) -> u64;

pub struct Runner<T> {
    timer: TimerFn<T>,
    diff: DiffFn<T>,
}

impl<'a, T> Runner<T> {
    pub fn new(timer: TimerFn<T>, diff: DiffFn<T>) -> Self {
        Runner { timer, diff }
    }

    pub fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Samples
        where Target: FnMut() -> Ret {

        let mut data = [0_u64; SAMPLE_SIZE];

        for i in 0..SAMPLE_SIZE {
            data[i] = self.run_loop(target);
        }

        Samples { name, data }
    }

    fn run_loop<Target, Ret>(&mut self, target: &mut Target) -> u64
        where Target: FnMut() -> Ret {

        let count = 10_000_u64;

        let start = (self.timer)();

        for _ in 0..count {
            black_box(target());
        }

        let end = (self.timer)();

        (self.diff)(&start, &end) / count
    }
}
