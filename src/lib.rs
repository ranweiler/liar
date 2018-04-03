#![deny(missing_docs)]

#![cfg_attr(asm, feature(asm))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(unix, feature = "std"))]
extern crate libc;

mod black_box;

#[cfg(feature = "std")] pub mod bencher;
#[cfg(feature = "std")] pub mod reporter;
#[cfg(feature = "std")] pub mod runner;
#[cfg(feature = "std")] pub mod timer;

pub mod no_std;

pub use self::black_box::black_box;

#[cfg(feature = "std")]
pub struct Sample<T> {
    pub name: &'static str,
    pub data: Vec<T>,
}

#[macro_export]
macro_rules! bench {
    ($name:ident, $body:expr) => {
        fn $name<R: Runner<S>, S>(b: &mut Bencher<R, S>) {
            b.run(|| $body);
        }
    };
    ($name:ident, $b:ident, $body:expr) => {
        fn $name<R: Runner<S>, S>($b: &mut Bencher<R, S>) {
            $body
        }
    };
}

#[macro_export]
macro_rules! add_bench {
    ($b:ident, $name:ident) => {
        $b.bench(stringify!($name), &mut $name);
    }
}
