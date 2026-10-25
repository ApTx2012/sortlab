use crate::traits::Sorter;

/// Quick sort implementation (iterative, Lomuto partition, median-of-three pivot).
pub struct Quick;

impl<T: Ord> Sorter<T> for Quick {
    fn sort<'a>(&self, data: &'a mut [T]) -> &'a mut [T] {
        let n = data.len();
        if n < 2 {
            return data;
        }
        // Explicit stack of (lo, hi) ranges avoids recursion depth issues.
        let mut stack: Vec<(usize, usize)> = vec![(0, n)];
        while let Some((lo, hi)) = stack.pop() {
            if hi - lo < 2 {
                continue;
            }
            // Median-of-three: order first / mid / last, use middle as pivot.
            let mid = lo + (hi - lo) / 2;
            if data[mid] < data[lo] {
                data.swap(mid, lo);
            }
            if data[hi - 1] < data[mid] {
                data.swap(hi - 1, mid);
                if data[mid] < data[lo] {
                    data.swap(mid, lo);
                }
            }
            data.swap(mid, hi - 1); // park pivot at the end
            // Lomuto partition around pivot at hi-1.
            let mut store = lo;
            for i in lo..hi - 1 {
                if data[i] <= data[hi - 1] {
                    data.swap(i, store);
                    store += 1;
                }
            }
            data.swap(store, hi - 1); // pivot to final position
            stack.push((lo, store));
            stack.push((store + 1, hi));
        }
        data
    }
}
