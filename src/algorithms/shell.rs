use crate::traits::Sorter;

/// Shell sort implementation with a classic gap sequence (Shell's 3^k sequence).
pub struct Shell;

impl<T: Ord> Sorter<T> for Shell {
    fn sort<'a>(&self, data: &'a mut [T]) -> &'a mut [T] {
        let n = data.len();
        if n < 2 {
            return data;
        }
        let mut gap = 1;
        while gap < n / 3 {
            gap = 3 * gap + 1;
        }
        while gap >= 1 {
            for i in gap..n {
                let mut j = i;
                while j >= gap && data[j - gap] > data[j] {
                    data.swap(j, j - gap);
                    j -= gap;
                }
            }
            gap /= 3;
        }
        data
    }
}
