//! Line-oriented reporter amenable to text processing tools.

use std::fmt::Debug;

use ::Sample;
use reporter::Reporter;
use runner::Round;


/// Reporter that outputs data as delimited text, with optional header.
pub struct LineReporter {
    delim: &'static str,
    header: bool,
}

impl LineReporter {
    /// Report results on lines, with columnns delimited by `delim`. If `header`
    /// is `true`, include a header line to label columns.
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

impl Reporter<Round> for LineReporter {
    fn report(&self, samples: &[Sample<Round>]) -> Result<(), ()> {
        if self.header {
            let l = ["name", "ns", "round_size"];
            println!("{}", l.join(self.delim));
        }

        for s in samples {
            for d in &s.data {
                let l = [s.name, &d.ns.to_string(), &d.size.to_string()];
                println!("{}", l.join(self.delim));
            }
        }
        Ok(())
    }
}
