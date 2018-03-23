pub mod fixed;
pub mod linear;

use ::Sample;


pub trait Runner<S> {
    fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<S>
        where Target: FnMut() -> Ret;
}

pub struct Round {
    pub ns: f64,
    pub size: usize,
}
