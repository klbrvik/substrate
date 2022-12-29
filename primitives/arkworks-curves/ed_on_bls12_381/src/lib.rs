#![cfg_attr(not(feature = "std"), no_std)]
#![deny(warnings, unused, future_incompatible, nonstandard_style, rust_2018_idioms)]
#![forbid(unsafe_code)]

//! This library implements a twisted Edwards curve whose base field is the
//! scalar field of the curve BLS12-381. This allows defining cryptographic
//! primitives that use elliptic curves over the scalar field of the latter
//! curve. This curve was generated by Sean Bowe, and is also known as [Jubjub](https://github.com/zkcrypto/jubjub).
//!
//! Curve information:
//! * Base field: q = 52435875175126190479447740508185965837690552500527637822603658699938581184513
//! * Scalar field: r = 6554484396890773809930967563523245729705921265872317281365359162392183254199
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 1
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = -1
//!    * d = -(10240/10241)

#[cfg(feature = "r1cs")]
pub use ark_ed_on_bls12_381::constraints::*;
mod curves;

pub use ark_ed_on_bls12_381::{Fq, FqConfig, Fr, FrConfig};
pub use curves::*;
