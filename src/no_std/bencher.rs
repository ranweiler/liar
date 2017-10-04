use no_std::runner::{DiffFn, Runner, Sample, TimerFn};


pub struct Bencher<'d, T> {
    data: &'d mut [u64],
    runner: Runner<T>,
}

impl<'d, T> Bencher<'d, T> {
    pub fn new(
        data: &'d mut [u64],
        timer: TimerFn<T>,
        diff: DiffFn<T>,
        round_size: usize,
    ) -> Self {

        Bencher {
            data,
            runner: Runner::new(round_size, timer, diff),
        }
    }

    pub fn run<Target, Ret>(&mut self, mut target: Target)
        where Target: FnMut() -> Ret {

        self.runner.run(&mut target, self.data);
    }

    pub fn bench<'s, Target>(
        &mut self,
        name: &'static str,
        target: &mut Target,
        data: &'s mut [u64],
    ) -> Sample<'s>
        where Target: FnMut(&mut Bencher<T>) {

        target(self);

        for i in 0..self.data.len() {
            data[i] = self.data[i];
        }

        Sample { name, data }
    }
}
