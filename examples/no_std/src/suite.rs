use liar::no_std::bencher::Bencher;

use report::report;
use time::{diff, time};

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


fn nop(b: &mut Bencher<u64>) {
    b.run(|| {});
}

fn zeroize(b: &mut Bencher<u64>) {
    const BUF_LEN: usize = 1024;
    let mut buf = [0u8; BUF_LEN];

    b.run(|| {
        for i in 0..BUF_LEN {
            buf[i] = 0;
        }
        buf
    });
}

fn ack(b: &mut Bencher<u64>) {
    b.run(|| {
        acker::mann(3, 2)
    });
}

const SAMPLE_SIZE: usize = 100;
const ROUNDS: usize = 1_000;

pub fn main() {
    let mut data = [0u64; SAMPLE_SIZE];
    let mut b = Bencher::new(&mut data, time, diff, ROUNDS);

    let mut out = [0u64; SAMPLE_SIZE];
    report(&b.bench("nop", &mut nop, &mut out));
    report(&b.bench("zeroize", &mut zeroize, &mut out));
    report(&b.bench("ack", &mut ack, &mut out));
}
