#[macro_use]
extern crate liar;

use liar::black_box;
use liar::bencher::Bencher;
use liar::runner::Runnable;


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
struct Nop;

impl Runnable<()> for Nop {
    fn body(&mut self) {}
}

// Succinct benchmark definition when no custom setup is needed.
bench!(ack, {
    acker::mann(black_box(3), black_box(2))
});

bench!(ack_black_box, {
    acker::mann(black_box(3), black_box(2))
});

fn main() {
    use liar::reporter::Reporter;
    use liar::reporter::line::LineReporter;
    use liar::runner::fixed;

    let r = fixed::FixedRunner::new(fixed::DEFAULT_ROUND_SIZE, fixed::DEFAULT_SAMPLE_SIZE);
    let mut b = Bencher::new(r);

    add_bench!(b, Nop);
    let nop_black_box = black_box(Nop);
    add_bench!(b, nop_black_box);
    add_bench!(b, ack);
    add_bench!(b, ack_black_box);

    let r = LineReporter::new("\t", false);
    r.report(&b.samples()).ok();
}
