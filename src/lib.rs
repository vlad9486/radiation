// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![cfg_attr(feature = "nightly", feature(min_specialization))]
#![no_std]

#[macro_use]
extern crate alloc;

extern crate self as radiation;

#[cfg(feature = "derive")]
pub use radiation_macros::{Absorb, Emit};

pub use nom;

mod absorb;
pub use self::absorb::{
    Absorb, AbsorbExt, ParseError, ParseErrorKind, Limit, LimitDescriptor, LimitError,
};

mod emit;
pub use self::emit::{RadiationBuffer, Emit};

#[cfg(all(test, feature = "derive"))]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynSized<T>(pub T);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection<C>(pub C);
