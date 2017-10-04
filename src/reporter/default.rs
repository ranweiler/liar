use runner::fixed::Sample;

pub struct Reporter {}

impl Reporter {
    pub fn new() -> Self {
        Reporter {}
    }

    pub fn report(&self, samples: &Vec<Sample<u64>>) {
        fn stats(s: &Sample<u64>) -> (f64, f64, f64) {
            let total = s.data.iter().sum::<u64>() as f64;
            let n = s.data.len() as f64;
            let mean = total / n;
            let var = s.data
                .iter()
                .map(|x| ((*x as f64) - mean).powi(2))
                .sum::<f64>() / (n - 1.0);
            let ssd = var.sqrt();
            let ssd_perc = ssd / mean * 100.0;
            (mean, ssd, ssd_perc)
        }

        for s in samples {
            let (mean, ssd, ssd_perc) = stats(&s);
            println!("[{:}]:\t{:} ± {:.3} ({:.3}%)", s.name, mean, ssd, ssd_perc);
        }
    }
}
