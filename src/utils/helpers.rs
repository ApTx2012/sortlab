/// Returns `true` if the slice is sorted in non-decreasing order.
pub fn is_sorted<T: Ord>(data: &[T]) -> bool {
    data.windows(2).all(|w| w[0] <= w[1])
}
