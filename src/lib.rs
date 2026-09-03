#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Sovereign Math — Locked Euclidean Division
//! Type-level NonZeroI32, branchless Euclidean adjustment, Kani/Prusti verified.

pub mod math;

pub use math::division_lock::{
    locked_divide_euclidean, locked_modulo_euclidean, DivModResult, LockedDivMod,
};
