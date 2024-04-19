use std::ops::{Add, RangeBounds, RangeInclusive};

use crate::{bounds_to_inclusive, Segtree};

///
/// Constant sized segment tree implementation
#[derive(Debug, Clone, PartialEq)]
pub struct StaticSegtree<T: Clone + PartialEq> {
    tree: Vec<T>,
    data_len: usize,
    merge_fn: fn(&T, &T) -> T,
    neutral_elem: T,
}

impl<T> Default for StaticSegtree<T>
where
    T: Add<T, Output = T> + Clone + Default + PartialEq,
{
    /// Constructs a zero sized segtree
    fn default() -> StaticSegtree<T> {
        StaticSegtree {
            tree: Vec::new(),
            data_len: 0,
            merge_fn: |a, b| a.clone() + b.clone(),
            neutral_elem: T::default(),
        }
    }
}

impl<T> StaticSegtree<T>
where
    T: Clone + PartialEq,
{
    /// Sets the given value on given index using given getter function
    /// which operates on the segtree Vec&lt;T&gt; and updates the segtree
    unsafe fn set_internal(
        &mut self,
        index: usize,
        value: T,
        getter: unsafe fn(&mut Vec<T>, usize) -> &mut T,
    ) {
        let mut crr = index + self.data_len;
        *getter(&mut self.tree, crr) = value;

        crr >>= 1;

        while crr != 0 {
            let l: T = getter(&mut self.tree, crr << 1).to_owned();
            let r: T = getter(&mut self.tree, crr << 1 | 1).to_owned();
            *getter(&mut self.tree, crr) = (self.merge_fn)(&l, &r);

            crr >>= 1;
        }
    }

    /// Queries given range on segtree using getter function which operates
    /// on the segtree Vec&lt;T&gt;
    unsafe fn query_internal(
        &self,
        range: RangeInclusive<usize>,
        getter: unsafe fn(&Vec<T>, usize) -> &T,
    ) -> T {
        let mut resl = self.neutral_elem.clone();
        let mut resr = self.neutral_elem.clone();

        let mut l = range.start() + self.data_len;
        let mut r = range.end() + self.data_len;

        while l < r {
            if l & 1 == 1 {
                resl = (self.merge_fn)(&resl, getter(&self.tree, l));
                l += 1;
            }
            if r & 1 == 0 {
                resr = (self.merge_fn)(getter(&self.tree, r), &resr);
                r -= 1;
            }

            l >>= 1;
            r >>= 1;
        }

        if l == r && l > 0 {
            resl = (self.merge_fn)(&resl, getter(&self.tree, l));
        }

        (self.merge_fn)(&resl, &resr)
    }
}

impl<T> Segtree<T> for StaticSegtree<T>
where
    T: Clone + PartialEq,
{
    fn from_slice(
        original: &[T],
        merge_fn: fn(&T, &T) -> T,
        neutral_elem: T,
    ) -> StaticSegtree<T> {
        let len = original.len();
        if len == 0 {
            return StaticSegtree {
                tree: Vec::new(),
                data_len: 0,
                merge_fn,
                neutral_elem,
            };
        }

        let mut tree: Vec<T> = Vec::with_capacity(2 * len);
        #[allow(clippy::uninit_vec)]
        unsafe {
            tree.set_len(2 * len)
        };

        tree[len..(2 * len)].clone_from_slice(original);
        for i in (1..len).rev() {
            tree[i] = merge_fn(&tree[i << 1], &tree[i << 1 | 1]);
        }
        tree[0] = neutral_elem.clone();

        StaticSegtree {
            tree,
            data_len: len,
            merge_fn,
            neutral_elem,
        }
    }

    fn len(&self) -> usize {
        self.data_len
    }

    fn is_empty(&self) -> bool {
        self.data_len == 0
    }

    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        self.set_internal(index, value, |v, i| v.get_unchecked_mut(i));
    }

    fn set(&mut self, index: usize, value: T) -> Option<()> {
        if index >= self.data_len {
            return None;
        }

        unsafe {
            self.set_internal(index, value, |v, i| v.get_mut(i).unwrap());
        }

        Some(())
    }

    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.tree.get_unchecked(index + self.data_len)
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.tree.get(index + self.data_len)
    }

    unsafe fn query_unchecked<R: RangeBounds<usize>>(&self, range: R) -> T {
        let range = bounds_to_inclusive(range, 0, self.data_len);
        self.query_internal(range, |v, i| v.get_unchecked(i))
    }

    fn query<R: RangeBounds<usize>>(&self, range: R) -> Option<T> {
        let range = bounds_to_inclusive(range, 0, self.data_len);
        if *range.end() >= self.data_len {
            None
        } else {
            unsafe {
                Some(self.query_internal(range, |v, i| v.get(i).unwrap()))
            }
        }
    }
}
