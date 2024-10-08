//! Criterion Benchmarks for ilog2 with and without the use of invariant! macro.

use assert_unchecked::assert_unchecked;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(never)]
fn ilog2_with_assert_unchecked(x: usize) -> u32 {
    unsafe {
        assert_unchecked!(x > 0, "x must be positive");
    }
    x.ilog2()
}

#[inline(never)]
fn ilog2_without_assert_unchecked(x: usize) -> u32 {
    debug_assert!(x > 0, "x must be positive");
    x.ilog2()
}

#[inline(never)]
fn ilog2_with_only_assert(x: usize) -> u32 {
    assert!(x > 0, "x must be positive");
    x.ilog2()
}

#[inline]
fn ilog2_with_assert_unchecked_inlined(x: usize) -> u32 {
    unsafe {
        assert_unchecked!(x > 0, "x must be positive");
    }
    x.ilog2()
}

#[inline]
fn ilog2_without_assert_unchecked_inlined(x: usize) -> u32 {
    debug_assert!(x > 0, "x must be positive");
    x.ilog2()
}

#[inline]
fn ilog2_with_only_assert_inlined(x: usize) -> u32 {
    assert!(x > 0, "x must be positive");
    x.ilog2()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ilog2_without_assert_unchecked", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_without_assert_unchecked(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_with_only_assert", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_with_only_assert(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_with_assert_unchecked", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_with_assert_unchecked(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_without_assert_unchecked_inlined", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_without_assert_unchecked_inlined(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_with_only_assert_inlined", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_with_only_assert_inlined(black_box(i));
            }
            xor
        })
    });
    c.bench_function("ilog2_with_assert_unchecked_inlined", |b| {
        b.iter(|| {
            let mut xor = 0;
            for i in 1..10_000 {
                xor ^= ilog2_with_assert_unchecked_inlined(black_box(i));
            }
            xor
        })
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
