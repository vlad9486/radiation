// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use core::marker::PhantomData;
use alloc::boxed::Box;

use super::core::Emit;

impl<T, W> Emit<W> for PhantomData<T>
where
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        buffer
    }
}

impl<W> Emit<W> for ()
where
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        buffer
    }
}

impl<W> Emit<W> for bool
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        if *self {
            buffer.extend(Some(0xff));
        } else {
            buffer.extend(Some(0x00));
        }
        buffer
    }
}

impl<W> Emit<W> for i8
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(Some(*self as u8));
        buffer
    }
}

impl<W> Emit<W> for u8
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(Some(*self));
        buffer
    }
}

impl<W> Emit<W> for i16
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.to_be_bytes());
        buffer
    }
}

impl<W> Emit<W> for u16
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.to_be_bytes());
        buffer
    }
}

impl<W> Emit<W> for i32
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.to_be_bytes());
        buffer
    }
}

impl<W> Emit<W> for u32
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.to_be_bytes());
        buffer
    }
}

impl<W> Emit<W> for i64
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.to_be_bytes());
        buffer
    }
}

impl<W> Emit<W> for u64
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.to_be_bytes());
        buffer
    }
}

impl<W> Emit<W> for f32
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.to_be_bytes());
        buffer
    }
}

impl<W> Emit<W> for f64
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.to_be_bytes());
        buffer
    }
}

impl<A, B, W> Emit<W> for (A, B)
where
    A: Emit<W>,
    B: Emit<W>,
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        let (a, b) = self;
        b.emit(a.emit(buffer))
    }
}

impl<const S: usize, W> Emit<W> for [u8; S]
where
    W: Extend<u8>,
{
    fn emit(&self, mut buffer: W) -> W {
        buffer.extend(self.iter().copied());
        buffer
    }
}

impl<T, W> Emit<W> for Option<T>
where
    T: Emit<W>,
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        match self {
            None => false.emit(buffer),
            Some(v) => v.emit(true.emit(buffer)),
        }
    }
}

impl<T, E, W> Emit<W> for Result<T, E>
where
    T: Emit<W>,
    E: Emit<W>,
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        match self {
            Ok(v) => v.emit(0xff.emit(buffer)),
            Err(v) => v.emit(0xfe.emit(buffer)),
        }
    }
}

impl<T, W> Emit<W> for Box<T>
where
    T: Emit<W>,
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        (**self).emit(buffer)
    }
}
