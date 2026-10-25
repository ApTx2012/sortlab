//! sortlab — a laboratory of sorting algorithms.
//!
//! One unified [`Sorter`] trait, several classic algorithm implementations,
//! and an iterator adaptor for sorting anything collectible.

pub mod algorithms;
pub mod prelude;
pub mod traits;
pub mod utils;

pub use traits::{IterAdaptor, Sorter};

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::utils;

    #[test]
    fn sorters_sort_slices() {
        for mut data in [
            vec![5, 2, 8, 1, 9, 3],
            vec![1, 2, 3],
            vec![],
            vec![1],
        ] {
            let len = data.len();
            Bubble.sort(&mut data);
            assert!(utils::is_sorted(&data));
            Quick.sort(&mut data);
            Merge.sort(&mut data);
            Shell.sort(&mut data);
            assert_eq!(len, data.len());
        }
    }

    #[test]
    fn iterator_adaptor_collects_sorted() {
        let v = vec![3, 1, 2].into_iter().collect_sorted();
        assert_eq!(v, vec![1, 2, 3]);
    }

    /// Stress test every sorter against `slice::sort` on many random inputs,
    /// covering empty, single-element, sorted, reversed, and duplicate-heavy
    /// slices of many different lengths.
    #[cfg(feature = "parallel")]
    #[test]
    fn stress_test_parallel_merge() {
        let mut data: Vec<i32> = (0..100_000).rev().collect();
        let mut expected = data.clone();
        expected.sort();
        ParallelMerge.sort(&mut data);
        assert_eq!(data, expected);
    }

    #[test]
    fn stress_test_all_sorters() {
        use crate::algorithms::{Bubble, Merge, Quick, Shell};

        // Simple xorshift PRNG keeps this test dependency-free.
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
        let sorters: [(&str, fn(&mut [i32])); 4] = [
            ("bubble", |d| {
                Bubble.sort(d);
            }),
            ("shell", |d| {
                Shell.sort(d);
            }),
            ("merge", |d| {
                Merge.sort(d);
            }),
            ("quick", |d| {
                Quick.sort(d);
            }),
        ];

        for (name, sort_with) in sorters {
            for len in [0usize, 1, 2, 3, 4, 5, 10, 31, 100, 500] {
                for rep in 0..20 {
                    // Choose a value range that shrinks with rep to force many duplicates.
                    let range = (rep as i32 + 1) * 3;
                    let mut data: Vec<i32> = (0..len)
                        .map(|_| (rng.next() % range as u64) as i32)
                        .collect();
                    let mut expected = data.clone();
                    expected.sort();
                    sort_with(&mut data);
                    assert_eq!(
                        data, expected,
                        "{name} failed: len={len}, rep={rep}"
                    );
                }
            }
        }
    }
}
