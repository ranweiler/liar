//! A portable timer powered by `std::time`.

use std::time::Instant;

use timer::Timer;


/// Wraps a `std::time::Instant`.
pub struct PortableTimer {
    instant: Instant,
}

impl PortableTimer {
    /// Construct a new timer, initialized with the current time.
    pub fn new() -> Self {
        let instant = Instant::now();

        PortableTimer { instant }
    }
}

impl Timer for PortableTimer {
    fn new_timer() -> Self {
        PortableTimer::new()
    }

    fn mark(&mut self) {
        self.instant = Instant::now();
    }

    fn since(&self, prev: &PortableTimer) -> f64 {
        let d = self.instant.duration_since(prev.instant);

        (d.as_secs() as f64) * 1e9 + (d.subsec_nanos() as f64)
    }
}
