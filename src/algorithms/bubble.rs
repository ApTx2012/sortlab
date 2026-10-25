use crate::traits::Sorter;

/// Bubble sort implementation.
pub struct Bubble;

impl<T: Ord> Sorter<T> for Bubble {
    fn sort<'a>(&self, data: &'a mut [T]) -> &'a mut [T] {
        let len = data.len();
        if len < 2 {
            return data;
        }
        for i in 0..len {
            let mut swapped = false;
            for j in 0..len - i - 1 {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
        data
    }
}
