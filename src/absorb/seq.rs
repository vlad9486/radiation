// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use core::str;
use alloc::{string::{String, ToString}, boxed::Box, vec::Vec};

use nom::{IResult, combinator, number, multi};

use super::{
    core::Absorb,
    error::{ParseError, ParseErrorKind},
    limit::Limit,
    DynSized, Collection,
};

impl<'pa> Absorb<'pa> for usize {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        let (input, size) = number::complete::be_u32(input)?;
        L::check(size as usize)
            .map(|size| (input, size))
            .map_err(|e| ParseErrorKind::Limit(e).error(input))
    }
}

impl<'pa> Absorb<'pa> for &'pa str {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map_res(multi::length_data(usize::absorb::<L>), str::from_utf8)(input)
    }
}

impl<'pa> Absorb<'pa> for String {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(<&str>::absorb::<L>, str::to_string)(input)
    }
}

impl<'pa, T> Absorb<'pa> for DynSized<T>
where
    T: Absorb<'pa>,
{
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map_parser(
            multi::length_data(usize::absorb::<L>),
            combinator::all_consuming(combinator::map(T::absorb::<L::Inner>, DynSized)),
        )(input)
    }
}

fn fold_parser<'pa, C, T, L>(
) -> impl FnMut(&'pa [u8]) -> IResult<&'pa [u8], C, ParseError<&'pa [u8]>>
where
    L: Limit,
    C: Default + Extend<T>,
    T: Absorb<'pa>,
{
    multi::fold_many_m_n(
        L::LOWER,
        L::UPPER,
        T::absorb::<L::Inner>,
        C::default,
        |mut list, item| {
            list.extend(Some(item));
            list
        },
    )
}

impl<'pa, T> Absorb<'pa> for Box<[T]>
where
    T: Absorb<'pa>,
{
    #[cfg(feature = "nightly")]
    default fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(fold_parser::<Vec<T>, T, L>(), Vec::into_boxed_slice)(input)
    }

    #[cfg(not(feature = "nightly"))]
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(fold_parser::<Vec<T>, T, L>(), Vec::into_boxed_slice)(input)
    }
}

#[cfg(feature = "nightly")]
impl<'pa> Absorb<'pa> for Box<[u8]> {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        L::check(input.len()).map_err(|e| ParseErrorKind::Limit(e).error(input))?;
        Ok((&[], input.into()))
    }
}

impl<'pa, T> Absorb<'pa> for Vec<T>
where
    T: Absorb<'pa>,
{
    #[cfg(feature = "nightly")]
    default fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map_parser(
            multi::length_data(usize::absorb::<L>),
            combinator::all_consuming(fold_parser::<Vec<T>, T, L>()),
        )(input)
    }

    #[cfg(not(feature = "nightly"))]
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map_parser(
            multi::length_data(usize::absorb::<L>),
            combinator::all_consuming(fold_parser::<Vec<T>, T, L>()),
        )(input)
    }
}

#[cfg(feature = "nightly")]
impl<'pa> Absorb<'pa> for Vec<u8> {
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(multi::length_data(usize::absorb::<L>), |s| s.to_vec())(input)
    }
}

impl<'pa, C> Absorb<'pa> for Collection<C>
where
    C: Default + IntoIterator + Extend<C::Item>,
    C::Item: Absorb<'pa>,
{
    fn absorb<L>(input: &'pa [u8]) -> IResult<&'pa [u8], Self, ParseError<&'pa [u8]>>
    where
        L: Limit,
    {
        combinator::map(fold_parser::<C, C::Item, L>(), Collection)(input)
    }
}
