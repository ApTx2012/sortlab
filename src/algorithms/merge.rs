use crate::traits::Sorter;
use std::mem::MaybeUninit;
use std::ptr;

/// Merge sort implementation (recursive, stable, O(n log n)).
pub struct Merge;

impl<T: Ord> Sorter<T> for Merge {
    fn sort<'a>(&self, data: &'a mut [T]) -> &'a mut [T] {
        merge_sort(data);
        data
    }
}

fn merge_sort<T: Ord>(data: &mut [T]) {
    let len = data.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut data[..mid]);
    merge_sort(&mut data[mid..]);
    merge_halves(data, mid);
}

/// Sorts `data` with sequential merge sort (crate-internal helper).
#[cfg(feature = "parallel")]
pub(crate) fn sort_slice<T: Ord>(data: &mut [T]) {
    merge_sort(data);
}

/// Merges the already-sorted halves `data[..mid]` and `data[mid..]` in place.
///
/// The left half is first *moved* into a scratch buffer so the merge can
/// run left-to-right in O(n). `MaybeUninit` is used to avoid requiring
/// `T: Clone`.
///
/// # Safety
///
/// Every slot of `buf` is initialized before it is read. Values are moved
/// out of the slice with `ptr::read`, so `ptr::write` must (and does)
/// overwrite every slot exactly once; `MaybeUninit` never drops its
/// payload, so no value is freed twice. Write index `w` never overtakes
/// the right-half read index `r` while the left half still has items,
/// so no unread value is ever clobbered.
pub(crate) fn merge_halves<T: Ord>(data: &mut [T], mid: usize) {
    let len = data.len();
    let mut buf: Vec<MaybeUninit<T>> = Vec::with_capacity(mid);
    unsafe {
        let src = data.as_mut_ptr();
        for i in 0..mid {
            buf.push(MaybeUninit::new(ptr::read(src.add(i))));
        }
        let (mut l, mut r, mut w) = (0usize, mid, 0usize);
        while l < mid && r < len {
            let take_left = *buf[l].as_ptr() <= data[r];
            if take_left {
                ptr::write(src.add(w), ptr::read(buf[l].as_ptr()));
                l += 1;
            } else {
                let val = ptr::read(src.add(r));
                ptr::write(src.add(w), val);
                r += 1;
            }
            w += 1;
        }
        while l < mid {
            ptr::write(src.add(w), ptr::read(buf[l].as_ptr()));
            l += 1;
            w += 1;
        }
        while r < len {
            let val = ptr::read(src.add(r));
            ptr::write(src.add(w), val);
            r += 1;
            w += 1;
        }
    }
}
