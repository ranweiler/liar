extern crate liar;

use liar::black_box;
use liar::bencher::Bencher;
use liar::runner::Runner;


mod acker {
    pub fn mann(m: usize, n: usize) -> usize {
        match m {
            0 => n + 1,
            _ => match n {
                0 => mann(m - 1, 1),
                n => mann(m - 1, mann(m, n - 1)),
            }
        }
    }
}


fn nop<R: Runner<S>, S>(b: &mut Bencher<R, S>) {
    b.run(|| {});
}

fn nop_black_box<R: Runner<S>, S>(b: &mut Bencher<R, S>) {
    b.run(|| (black_box(3), black_box(2)));
}

fn ack<R: Runner<S>, S>(b: &mut Bencher<R, S>) {
    b.run(|| {
        acker::mann(3, 2)
    });
}

fn ack_black_box<R: Runner<S>, S>(b: &mut Bencher<R, S>) {
    b.run(|| {
        acker::mann(black_box(3), black_box(2))
    });
}

fn main() {
    use liar::reporter::Reporter;
    use liar::runner::fixed;

    let r = fixed::FixedRunner::new(fixed::DEFAULT_ROUND_SIZE, fixed::DEFAULT_SAMPLE_SIZE);
    let mut b = Bencher::new(r);

    b.bench("nop", &mut nop);
    b.bench("nop_black_box", &mut nop_black_box);
    b.bench("ack", &mut ack);
    b.bench("ack_black_box", &mut ack_black_box);

    let r = Reporter::new();
    r.report(&b.samples());
}
