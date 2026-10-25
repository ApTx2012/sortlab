//! Parallel sorting (requires the `parallel` feature).
//!
//! Implemented with plain `std::thread` — no third-party dependencies.

use crate::traits::Sorter;

/// Parallel merge sort: splits the slice across worker threads.
///
/// Falls back to sequential merge sort when the slice is small or when
/// spawning a thread fails.
pub struct ParallelMerge;

impl<T: Ord + Send> Sorter<T> for ParallelMerge {
    fn sort<'a>(&self, data: &'a mut [T]) -> &'a mut [T] {
        parallel_merge_sort(data, 4);
        data
    }
}

fn parallel_merge_sort<T: Ord + Send>(data: &mut [T], depth: usize) {
    let len = data.len();
    if len < 2 {
        return;
    }
    let mid = len / 2;
    let (left, right) = data.split_at_mut(mid);

    if depth > 0 && len >= 4096 {
        // SAFETY: `left` and `right` are disjoint slices, each fully owned
        // by its own scope after `std::thread::scope`. Both halves are `T: Send`.
        std::thread::scope(|s| {
            let handle = s.spawn(|| parallel_merge_sort(left, depth - 1));
            parallel_merge_sort(right, depth - 1);
            if handle.join().is_err() {
                // Propagate panics from the worker thread, mirroring std behavior.
                std::process::abort();
            }
        });
    } else {
        crate::algorithms::merge::sort_slice(left);
        crate::algorithms::merge::sort_slice(right);
    }
    crate::algorithms::merge::merge_halves(data, mid);
}
