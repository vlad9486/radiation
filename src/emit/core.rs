// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

pub trait RadiationBuffer {
    fn pos(&self) -> usize;
    fn write_at(&mut self, pos: usize, data: &[u8]);
}

impl RadiationBuffer for Vec<u8> {
    fn pos(&self) -> usize {
        self.len()
    }

    fn write_at(&mut self, pos: usize, data: &[u8]) {
        self[pos..data.len()].clone_from_slice(data);
    }
}

pub trait Emit<W>
where
    W: Extend<u8>,
{
    #[must_use]
    fn emit(&self, buffer: W) -> W;
}
