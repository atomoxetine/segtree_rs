use std::ops::{Add, Bound, RangeBounds, RangeInclusive, Sub};

pub mod static_segtree;

/// Segment Tree trait with common methods
pub trait Segtree<T: Clone>: Clone + PartialEq {

    /// Creates segtree from slice of type T, with a specified
    /// merge function (allowing for non commutative operations)
    /// and a neutral element (usually 0 or 1 when T is an integer)
    ///
    /// Make sure your merge function is associative and pure, otherwise
    /// it will not work
    fn from_slice(
        original: &[T],
        merge_fn: fn(&T, &T) -> T,
        neutral_elem: T,
    ) -> Self;

    /// Returns the length of raw data
    fn len(&self) -> usize;

    /// Returns true if there is no data
    fn is_empty(&self) -> bool;

    /// Returns an immutable reference to data at specified
    /// index without doing bounds checking
    ///
    /// # Safety
    ///
    /// This method is unsafe, as it results in undefined behaviour if run
    /// with index out of bounds
    unsafe fn get_unchecked(&self, index: usize) -> &T;

    /// Returns an immutable reference to data at specified
    /// index while doing bounds checking
    ///
    /// Will return [None] if the index is out of bounds
    fn get(&self, index: usize) -> Option<&T>;

    /// Sets the value on desired index without doing bounds checking
    ///
    /// # Safety
    ///
    /// This method is unsafe, as it results in undefined behaviour if run
    /// with index out of bounds
    unsafe fn set_unchecked(&mut self, index: usize, value: T);

    /// Sets the value on desired index while doing bounds checking
    ///
    /// Will return [None] if the index is out of bounds
    fn set(&mut self, index: usize, value: T) -> Option<()>;

    /// Queries for data in range without doing bounds checking
    ///
    /// # Safety
    ///
    /// This method is unsafe, as it results in undefined behaviour if run
    /// with index out of bounds
    unsafe fn query_unchecked<R: RangeBounds<usize>>(&self, range: R) -> T;

    /// Queries for data in range while doing bounds checking
    ///
    /// Will return [None] if the index is out of bounds
    fn query<R: RangeBounds<usize>>(&self, range: R) -> Option<T>;
}

pub fn bounds_to_inclusive<
    T: Copy + From<u8> + Add<Output = T> + Sub<Output = T>,
    R: RangeBounds<T>,
>(
    range: R,
    min: T,
    max: T,
) -> RangeInclusive<T> {
    let start = match range.start_bound() {
        Bound::Included(s) => *s,
        Bound::Excluded(s) => *s + 1.into(),
        Bound::Unbounded => min,
    };

    let end = match range.end_bound() {
        Bound::Included(e) => *e,
        Bound::Excluded(e) => *e - 1.into(),
        Bound::Unbounded => max,
    };

    start..=end
}
