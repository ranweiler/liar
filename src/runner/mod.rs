pub mod fixed;

use ::Sample;


pub trait Runner<S> {
    fn run<Target, Ret>(&mut self, name: &'static str, target: &mut Target) -> Sample<S>
        where Target: FnMut() -> Ret;
}
