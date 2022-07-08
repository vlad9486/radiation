// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use super::{
    core::{RadiationBuffer, Emit},
    DynSized, Collection,
};

impl<W> Emit<W> for usize
where
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        (*self as u32).emit(buffer)
    }
}

impl<W> Emit<W> for str
where
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        let bytes = self.as_bytes();
        let mut buffer = bytes.len().emit(buffer);
        buffer.extend(bytes.iter().copied());
        buffer
    }
}

impl<W> Emit<W> for String
where
    W: Extend<u8>,
{
    fn emit(&self, buffer: W) -> W {
        (**self).emit(buffer)
    }
}

impl<T, W> Emit<W> for DynSized<T>
where
    T: Emit<W>,
    W: Extend<u8> + RadiationBuffer,
{
    fn emit(&self, buffer: W) -> W {
        let pos = buffer.pos();
        let mut buffer = self.0.emit(0usize.emit(buffer));
        let len = buffer.pos() - pos - 4;
        buffer.write_at(pos, &len.to_be_bytes());
        buffer
    }
}

impl<T, W> Emit<W> for Box<[T]>
where
    T: Emit<W>,
    W: Extend<u8> + RadiationBuffer,
{
    fn emit(&self, mut buffer: W) -> W {
        for i in self.iter() {
            buffer = i.emit(buffer);
        }
        buffer
    }
}

impl<T, W> Emit<W> for Vec<T>
where
    T: Emit<W>,
    W: Extend<u8> + RadiationBuffer,
{
    fn emit(&self, mut buffer: W) -> W {
        let pos = buffer.pos();
        for v in self {
            buffer = v.emit(buffer);
        }
        let len = buffer.pos() - pos - 4;
        buffer.write_at(pos, &len.to_be_bytes());
        buffer
    }
}

impl<C, W> Emit<W> for Collection<C>
where
    C: IntoIterator + Clone,
    C::Item: Emit<W>,
    W: Extend<u8> + RadiationBuffer,
{
    fn emit(&self, mut buffer: W) -> W {
        for v in C::clone(&self.0) {
            buffer = v.emit(buffer);
        }
        buffer
    }
}
