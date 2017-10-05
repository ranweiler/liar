#[macro_use]
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

// Manual benchmark definition.
fn nop<R: Runner<S>, S>(b: &mut Bencher<R, S>) {
    b.run(|| {});
}

// Benchmark definition with macro, allowing custom setup.
bench!(nop_black_box, b, {
    let m = 3;
    let n = 2;

    b.run(|| (black_box(m), black_box(n)));
});

// Succinct benchmark definition when no custom setup is needed.
bench!(ack, {
    acker::mann(black_box(3), black_box(2))
});

bench!(ack_black_box, {
    acker::mann(black_box(3), black_box(2))
});

fn main() {
    use liar::reporter::Reporter;
    use liar::runner::fixed;

    let r = fixed::FixedRunner::new(fixed::DEFAULT_ROUND_SIZE, fixed::DEFAULT_SAMPLE_SIZE);
    let mut b = Bencher::new(r);

    add_bench!(b, nop);
    add_bench!(b, nop_black_box);
    add_bench!(b, ack);
    add_bench!(b, ack_black_box);

    let r = Reporter::new();
    r.report(&b.samples());
}
