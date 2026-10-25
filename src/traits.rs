/// A sorting strategy: sorts a slice in place and returns
/// the same slice to allow chained use.
pub trait Sorter<T: Ord> {
    /// Sorts `data` in place (non-decreasing order) and returns
    /// the sorted mutable slice.
    fn sort<'a>(&self, data: &'a mut [T]) -> &'a mut [T];
}

/// Iterator adaptor: turns any iterator into a sorted `Vec`.
pub trait IterAdaptor<T: Ord> {
    /// Collects all items from the iterator and returns them
    /// sorted in non-decreasing order.
    fn collect_sorted(self) -> Vec<T>
    where
        Self: Sized;
}

impl<T: Ord, I: Iterator<Item = T>> IterAdaptor<T> for I {
    fn collect_sorted(self) -> Vec<T> {
        let mut v: Vec<T> = self.collect();
        v.sort();
        v
    }
}
