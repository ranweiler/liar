use std::fmt::Debug;

use ::Sample;
use reporter::Reporter;


pub struct LineReporter {
    delim: &'static str,
    header: bool,
}

impl LineReporter {
    pub fn new(delim: &'static str, header: bool) -> Self {
        LineReporter { delim, header }
    }
}

impl<S: Debug> Reporter<S> for LineReporter {
    fn report(&self, samples: &[Sample<S>]) -> Result<(), ()> {
        if self.header {
            println!("name{}data", self.delim);
        }

        for s in samples {
            for d in &s.data {
                println!("{}{}{:?}", s.name, self.delim, d);
            }
        }
        Ok(())
    }
}
