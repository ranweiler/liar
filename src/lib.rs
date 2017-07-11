#![cfg_attr(asm, feature(asm))]
#![cfg_attr(not(feature = "std"), no_std)]

mod black_box;
pub mod bencher;
pub mod reporter;
pub mod runner;

pub use self::black_box::black_box;
