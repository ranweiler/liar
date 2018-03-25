pub trait Timer {
    fn new_timer() -> Self;
    fn mark(&mut self);
    fn since(&self, prev: &Self) -> f64;
}

pub mod portable;

#[cfg(all(unix, feature = "std"))]
pub mod posix;
