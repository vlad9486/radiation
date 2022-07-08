// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use super::{AbsorbExt, Absorb, ParseError, ParseErrorKind, Emit};

#[derive(Debug, PartialEq, Eq, Absorb, Emit)]
struct SomeStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

#[test]
fn trivial_struct() {
    let foo = SomeStruct::absorb_ext(b"\x12\x23\x34\x45\x56\x67\x78").unwrap();
    assert_eq!(foo.emit(vec![]), b"\x12\x23\x34\x45\x56\x67\x78");
    assert_eq!(
        foo,
        SomeStruct {
            a: 0x12,
            b: 0x2334,
            c: 0x45566778,
        }
    );
}

#[derive(Debug, PartialEq, Eq, Absorb, Emit)]
#[tag(u8)]
enum SomeEnum {
    #[tag(1)]
    A {
        one: u8,
        two: u8,
        // specify a function to parse the field with
        #[custom_absorb(absorb)]
        #[custom_emit(emit)]
        three: u16,
    },
    // use `FromStr` implementation to parse the value from `str`
    B(#[as_str] u16),
    C(u32),
}

// custom parser, parse u8 and square the result
fn absorb<'pa>(input: &'pa [u8]) -> nom::IResult<&'pa [u8], u16, ParseError<&'pa [u8]>> {
    crate::nom::combinator::map(u8::absorb::<()>, |a| a as u16 * a as u16)(input)
}

fn emit<W>(value: &u16, buffer: W) -> W
where
    W: Extend<u8>,
{
    let a = (*value as f32).sqrt() as u8;
    a.emit(buffer)
}

#[test]
fn trivial_enum() {
    let a = SomeEnum::absorb_ext(b"\x01\xcc\xdd\x12").unwrap();
    assert_eq!(a.emit(vec![]), b"\x01\xcc\xdd\x12");
    assert_eq!(
        a,
        SomeEnum::A {
            one: 0xcc,
            two: 0xdd,
            three: 0x12 * 0x12,
        }
    );

    let b = SomeEnum::absorb_ext(b"\x02\x00\x00\x00\x0512345").unwrap();
    assert_eq!(b.emit(vec![]), b"\x02\x00\x00\x00\x0512345");
    assert_eq!(b, SomeEnum::B(12345));

    let c = SomeEnum::absorb_ext(b"\x03\x12\x34\xab\xcd").unwrap();
    assert_eq!(c.emit(vec![]), b"\x03\x12\x34\xab\xcd");
    assert_eq!(c, SomeEnum::C(0x1234abcd));

    let err = SomeEnum::absorb_ext(b"\x04").unwrap_err();
    if let nom::Err::Error(err) = &err {
        if let ParseErrorKind::UnknownTag { hint, .. } = &err.kind {
            if *hint == stringify!(SomeEnum) {
                return;
            }
        }
    }
    panic!("unexpected error {}", err);
}
