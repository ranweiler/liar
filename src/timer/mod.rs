//! User-configurable timing measurement.

/// Can be used to mark two points in time, and from two points, compute a
/// spanned duration.
pub trait Timer {
    /// Construct a new timer.
    fn new_timer() -> Self;

    /// Mark the current time.
    fn mark(&mut self);

    /// Compute the time spanned by two timers.
    fn since(&self, prev: &Self) -> f64;
}

pub mod portable;

#[cfg(all(unix, feature = "std"))]
pub mod posix;
