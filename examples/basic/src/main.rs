extern crate liar;

use liar::{black_box, Bencher};


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

fn nop(b: &mut Bencher) {
    b.run(|| {});
}

fn nop_black_box(b: &mut Bencher) {
    b.run(|| (black_box(3), black_box(2)));
}

fn ack(b: &mut Bencher) {
    b.run(|| {
        acker::mann(3, 2)
    });
}

fn ack_black_box(b: &mut Bencher) {
    b.run(|| {
        acker::mann(black_box(3), black_box(2))
    });
}

fn main() {
    let mut b = liar::Bencher::new();

    b.bench("nop", &mut nop);
    b.bench("nop_black_box", &mut nop_black_box);
    b.bench("ack", &mut ack);
    b.bench("ack_black_box", &mut ack_black_box);

    let r = liar::reporter::Reporter::new();
    r.report(&b.samples());
}
