pub mod line;

use Sample;

pub trait Reporter<S> {
    fn report(&self, samples: &[Sample<S>]) -> Result<(), ()>;
}
