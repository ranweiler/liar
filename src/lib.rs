#![cfg_attr(asm, feature(asm))]
#![cfg_attr(not(feature = "std"), no_std)]

mod black_box;

#[cfg(feature = "std")] pub mod bencher;
#[cfg(feature = "std")] pub mod reporter;
#[cfg(feature = "std")] pub mod runner;

pub mod no_std;

pub use self::black_box::black_box;

#[cfg(feature = "std")]
pub struct Sample<T> {
    pub name: &'static str,
    pub data: Vec<T>,
}
