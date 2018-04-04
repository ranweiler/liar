//! Flexible, stand-alone benchmarking.

#![deny(missing_docs)]

#![cfg_attr(asm, feature(asm))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(unix, feature = "std"))]
extern crate libc;

mod black_box;

#[cfg(feature = "std")] pub mod bencher;
#[cfg(feature = "std")] pub mod benchmark;
#[cfg(feature = "std")] pub mod reporter;
#[cfg(feature = "std")] pub mod runner;
#[cfg(feature = "std")] pub mod timer;

#[allow(missing_docs)]
pub mod no_std;

pub use self::black_box::black_box;

/// Data sample for a specific benchmark, parametric in the sample unit.
#[cfg(feature = "std")]
pub struct Sample<T> {
    /// Name of the benchmark.
    pub name: String,

    /// Recorded data.
    pub data: Vec<T>,
}

#[macro_export]
macro_rules! bench {
    ($name: ident, $body: expr) => {
        #[allow(non_camel_case_types)]
        struct $name;

        impl Benchmark<()> for $name {
            fn name(&self) -> String {
                stringify!($name).to_string()
            }

            fn setup(&mut self) {}

            fn target(&mut self) {
                black_box({
                    $body
                });
            }

            fn teardown(&mut self) {}
        }
    };
}

#[macro_export]
macro_rules! run_bench {
    ($b:ident, $name:ident) => {
        $b.run(&mut $name {});
    }
}
