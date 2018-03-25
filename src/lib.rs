#![cfg_attr(asm, feature(asm))]
#![cfg_attr(not(feature = "std"), no_std)]

mod black_box;

#[cfg(feature = "std")]
pub mod bencher;
#[cfg(feature = "std")]
pub mod reporter;
#[cfg(feature = "std")]
pub mod runner;

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
        #[allow(non_camel_case_types)]
        struct $name;
        impl Runnable<()> for $name {
            fn body(&mut self) {
                 $body;
            }
        }
    };
}

#[macro_export]
macro_rules! add_bench {
    ($b:ident, $name:ident) => {
        $b.bench(stringify!($name), $name);
    }
}
