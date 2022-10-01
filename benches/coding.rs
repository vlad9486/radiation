// Copyright 2021 Vladislav Melnik
// SPDX-License-Identifier: MIT

#![forbid(unsafe_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use radiation::{AbsorbExt, Emit};

fn coding(c: &mut Criterion) {
    #[inline(never)]
    fn f(input: Vec<u8>) -> Vec<u8> {
        Box::<[u8]>::absorb_ext(&input)
            .expect("trivial structure")
            .to_vec()
    }

    c.bench_function("coding 64kb", |b| {
        b.iter(|| f(black_box(vec![0x12; 0x10000])))
    });
}

fn coding_simple(c: &mut Criterion) {
    #[inline(never)]
    fn f(input: Vec<u8>) -> Vec<u8> {
        input
    }

    c.bench_function("clone 64kb", |b| {
        b.iter(|| f(black_box(vec![0x12; 0x10000])))
    });
}

fn emitting(c: &mut Criterion) {
    #[inline(never)]
    fn f(input: Vec<u8>) -> Vec<u8> {
        input.chain(vec![])
    }

    c.bench_function("emitting 64kb", |b| {
        b.iter(|| f(black_box(vec![0x12; 0x10000])))
    });
}

criterion_group!(benches, coding, coding_simple, emitting);
criterion_main!(benches);
