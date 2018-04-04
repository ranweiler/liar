#[macro_use]
extern crate liar;

use liar::black_box;
use liar::bencher::Bencher;
use liar::benchmark::Benchmark;


mod fac {
    pub fn rec(n: u64) -> u64 {
        if n == 0 {
            return 1;
        }

        n * rec(n - 1)
    }

    pub fn iter(n: u64) -> u64 {
        let mut acc = 1;
        let mut k = n;
        while k != 0 {
            acc *= n;
            k -= 1;
        }
        acc
    }
}


// Manual benchmark definition.
#[allow(non_camel_case_types)]
struct bench_fac_rec;

impl Benchmark<u64> for bench_fac_rec {
    fn name(&self) -> String {
        "bench_fac_rec".to_string()
    }

    fn setup(&mut self) {}

    fn target(&mut self) -> u64 {
        fac::rec(black_box(20))
    }

    fn teardown(&mut self) {}
}

// Example of equivalent definition via macro.
bench!(bench_fac_iter, {
    black_box(fac::iter(black_box(20)));
});

struct BenchFac {
    arg: u64,
}

impl Benchmark<u64> for BenchFac {
    fn name(&self) -> String {
        "bench_fac_with_setup".to_string()
    }

    fn setup(&mut self) {
        self.arg = black_box(20);
    }

    fn target(&mut self) -> u64 {
        fac::iter(self.arg)
    }

    fn teardown(&mut self) {}
}


fn main() {
    use liar::reporter::Reporter;
    use liar::reporter::line::LineReporter;
    use liar::runner::fixed;
    use liar::timer::posix::CPUTimer;

    let r = fixed::FixedRunner::<CPUTimer>::new(
        fixed::DEFAULT_ROUND_SIZE,
        fixed::DEFAULT_SAMPLE_SIZE
    );
    let mut b = Bencher::new(r);

    run_bench!(b, bench_fac_rec);
    run_bench!(b, bench_fac_iter);
    b.run(&mut BenchFac { arg: 0 });

    let r = LineReporter::new("\t", false);
    r.report(&b.samples()).ok();
}
