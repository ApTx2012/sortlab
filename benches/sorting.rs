//! Benchmarks for all sorters, compared against `slice::sort`.
//!
//! Run with `cargo bench`. Criterion stores previous results and reports
//! regressions automatically on subsequent runs.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use sortlab::prelude::*;

/// Input distributions: random, sorted, reversed, and many-duplicates.
fn make_data(kind: &str, n: usize) -> Vec<u64> {
    // xorshift PRNG, deterministic across runs.
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
    }
    let mut rng = Rng(0x9E3779B97F4A7C15);
    match kind {
        "random" => (0..n).map(|_| rng.next()).collect(),
        "sorted" => (0..n as u64).collect(),
        "reversed" => (0..n as u64).rev().collect(),
        "dupes" => (0..n).map(|_| rng.next() % 10).collect(),
        _ => unreachable!(),
    }
}

const SIZES: [usize; 3] = [1_000, 10_000, 100_000];
const KINDS: [&str; 4] = ["random", "sorted", "reversed", "dupes"];

fn bench_sorters(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorters");
    group.sample_size(10);

    for &kind in KINDS.iter() {
        for &n in SIZES.iter() {
            // Bubble sort is O(n^2); only bench it on the small size,
            // otherwise it would dominate the runtime for no insight.
            let bench_bubble = n <= 1_000;

            let data = make_data(kind, n);

            if bench_bubble {
                group.bench_with_input(
                    BenchmarkId::new("bubble", format!("{kind}/{n}")),
                    &data,
                    |b, d| {
                        b.iter(|| {
                            let mut v = d.clone();
                            Bubble.sort(black_box(&mut v));
                        })
                    },
                );
            }
            group.bench_with_input(
                BenchmarkId::new("shell", format!("{kind}/{n}")),
                &data,
                |b, d| {
                    b.iter(|| {
                        let mut v = d.clone();
                        Shell.sort(black_box(&mut v));
                    })
                },
            );
            group.bench_with_input(
                BenchmarkId::new("merge", format!("{kind}/{n}")),
                &data,
                |b, d| {
                    b.iter(|| {
                        let mut v = d.clone();
                        Merge.sort(black_box(&mut v));
                    })
                },
            );
            group.bench_with_input(
                BenchmarkId::new("quick", format!("{kind}/{n}")),
                &data,
                |b, d| {
                    b.iter(|| {
                        let mut v = d.clone();
                        Quick.sort(black_box(&mut v));
                    })
                },
            );
            group.bench_with_input(
                BenchmarkId::new("std", format!("{kind}/{n}")),
                &data,
                |b, d| {
                    b.iter(|| {
                        let mut v = d.clone();
                        black_box(&mut v).sort();
                    })
                },
            );
        }
    }
    group.finish();
}

fn bench_iterator_adaptor(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterator_adaptor");
    group.sample_size(10);

    for &n in SIZES.iter() {
        let data = make_data("random", n);
        group.bench_with_input(BenchmarkId::new("collect_sorted", n), &data, |b, d| {
            b.iter(|| {
                let v: Vec<u64> = black_box(d.iter().copied()).collect_sorted();
                black_box(v);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_sorters, bench_iterator_adaptor);
criterion_main!(benches);
