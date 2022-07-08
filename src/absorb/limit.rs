// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use core::fmt;

#[derive(Debug)]
pub struct LimitError {
    description: &'static str,
    minimum: usize,
    maximum: usize,
    actual: usize,
}

impl fmt::Display for LimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LimitError { description, minimum, maximum, actual } = self;
        if *actual > *maximum {
            write!(f, "{description} falls outside the allowed maximum {actual} > {maximum}")
        } else if *actual < *minimum {
            write!(f, "{description} falls below the allowed minimum {actual} < {minimum}")
        } else {
            Ok(())
        }
    }
}

pub trait Limit {
    type Inner: Limit;

    const LOWER: usize;

    const UPPER: usize;

    const DESCRIPTION: &'static str;

    fn check(size: usize) -> Result<usize, LimitError> {
        if size <= Self::UPPER && size >= Self::LOWER {
            Ok(size)
        } else {
            Err(LimitError {
                description: Self::DESCRIPTION,
                minimum: Self::LOWER,
                maximum: Self::UPPER,
                actual: size,
            })
        }
    }
}

impl Limit for () {
    type Inner = ();

    const LOWER: usize = 0;

    const UPPER: usize = usize::MAX;

    const DESCRIPTION: &'static str = "unlimited";

    fn check(size: usize) -> Result<usize, LimitError> {
        Ok(size)
    }
}

pub struct LimitDescriptor<L, const LOWER: usize, const UPPER: usize>(L);

impl<L, const LOWER: usize, const UPPER: usize> Limit for LimitDescriptor<L, LOWER, UPPER>
where
    L: Limit,
{
    type Inner = L;

    const LOWER: usize = LOWER;

    const UPPER: usize = UPPER;

    const DESCRIPTION: &'static str = "wait for issue 44580 (adt_const_params)";
}
