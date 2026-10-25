use crate::traits::Sorter;
use crate::utils::is_sorted;

/// Quick sort implementation (iterative, 3-way partition, median-of-three
/// pivot, insertion sort for small ranges, ordered-run fast paths).
///
/// The 3-way ("Dutch national flag") partition splits the range into
/// `< pivot`, `== pivot`, and `> pivot` segments. Inputs with many
/// duplicate keys — the worst case for a classic Lomuto/Hoare partition,
/// which degrades to O(n²) there — are handled in near-linear time.
///
/// Two fast paths guard each partition:
/// * ranges at or below [`INSERTION_THRESHOLD`] finish with insertion sort;
/// * ascending / descending runs are detected with an early-exit scan and
///   returned (or reversed) immediately — without this, the pivot swap
///   would scatter an already-ordered range and every level below would
///   pay for the lost cache locality.
pub struct Quick;

/// Below this size, insertion sort beats the partitioning machinery.
const INSERTION_THRESHOLD: usize = 24;

impl<T: Ord> Sorter<T> for Quick {
    fn sort<'a>(&self, data: &'a mut [T]) -> &'a mut [T] {
        let n = data.len();
        if n < 2 {
            return data;
        }
        // Explicit stack of (lo, hi) ranges avoids recursion depth issues.
        let mut stack: Vec<(usize, usize)> = vec![(0, n)];
        while let Some((lo, hi)) = stack.pop() {
            let len = hi - lo;
            if len < 2 {
                continue;
            }
            // Small ranges: insertion sort directly, skip partitioning.
            if len <= INSERTION_THRESHOLD {
                insertion_sort(&mut data[lo..hi]);
                continue;
            }
            // Already ordered? Done. Strictly descending? Reverse and done.
            // Both scans short-circuit on the first inversion, so on random
            // data they cost a couple of comparisons per partition.
            if is_sorted(&data[lo..hi]) {
                continue;
            }
            if is_descending(&data[lo..hi]) {
                data[lo..hi].reverse();
                continue;
            }

            // Median-of-three: order first / mid / last, use the middle
            // value as pivot to avoid O(n²) on sorted/reversed inputs.
            let mid = lo + len / 2;
            if data[mid] < data[lo] {
                data.swap(mid, lo);
            }
            if data[hi - 1] < data[mid] {
                data.swap(hi - 1, mid);
                if data[mid] < data[lo] {
                    data.swap(mid, lo);
                }
            }
            data.swap(mid, lo); // pivot now at `lo`

            // 3-way partition over data[lo+1..hi] (invariants):
            //   data[lo   .. lt]  <  pivot      (lt starts at lo+1: slot `lo` holds pivot)
            //   data[lt  .. i  ] ==  pivot
            //   data[i   .. gt ]  unexplored
            //   data[gt  .. hi ]  >  pivot
            let (mut lt, mut i, mut gt) = (lo + 1, lo + 1, hi);
            while i < gt {
                if data[i] < data[lo] {
                    data.swap(i, lt);
                    lt += 1;
                    i += 1;
                } else if data[i] > data[lo] {
                    gt -= 1;
                    data.swap(i, gt);
                } else {
                    i += 1;
                }
            }
            // Move the pivot from `lo` into the `==` segment: swapping with
            // `lt - 1` (the last `<` slot) excludes it from the left range
            // below. Order inside the three segments is irrelevant;
            // recursing restores it.
            data.swap(lo, lt - 1);
            // Recurse into the two strict segments; the `==` middle is done.
            stack.push((lo, lt - 1));
            stack.push((gt, hi));
        }
        data
    }
}

/// Returns `true` if the slice is in strictly (or equally) descending order.
fn is_descending<T: Ord>(data: &[T]) -> bool {
    data.windows(2).all(|w| w[0] >= w[1])
}

/// Sorts a small slice with insertion sort (stable, O(n²) but with tiny
/// constants — the right tool for ranges of a few dozen elements).
fn insertion_sort<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            j -= 1;
        }
    }
}
