pub trait Timer {
    fn new_timer() -> Self;
    fn mark(&mut self);
    fn since(&self, prev: &Self) -> f64;
}

#[cfg(all(unix, feature = "std"))]
pub mod posix;
