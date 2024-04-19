use std::ops::Add;

#[derive(Debug)]
pub struct StaticSegtree<T: Clone> {
    tree: Vec<T>,
    data_len: usize,
    merge_fn: fn(&T, &T) -> T,
    default: T,
}

impl<T> StaticSegtree<T>
where
    T: Add<T, Output = T> + Clone + Default,
{
    pub fn new() -> StaticSegtree<T> {
        StaticSegtree {
            tree: Vec::new(),
            data_len: 0,
            merge_fn: |a, b| a.clone() + b.clone(),
            default: T::default(),
        }
    }
}

pub enum SegtreeAccessError {
    IndexOutOfBounds,
}

pub enum SegtreeRangeError {
    RangeOutOfBounds,
    InvalidRange,
}

impl<T: Clone> StaticSegtree<T> {
    pub fn from_vec(
        original: &Vec<T>,
        merge_fn: fn(&T, &T) -> T,
        default: T,
    ) -> StaticSegtree<T> {
        let len = original.len();
        let mut tree: Vec<T> = Vec::with_capacity(2 * len);
        unsafe { tree.set_len(2 * len) };

        for i in 0..len {
            tree[i + len] = original[i].clone();
        }

        for i in (1..len).rev() {
            tree[i] = merge_fn(&tree[i << 1], &tree[i << 1 | 1]);
        }

        StaticSegtree {
            tree,
            data_len: len,
            merge_fn,
            default,
        }
    }

    pub fn len(&self) -> usize {
        self.data_len
    }

    pub fn set(&mut self, index: usize, value: T) {
        let mut crr = index + self.data_len;
        self.tree[crr] = value;

        crr >>= 1;

        while crr != 0 {
            self.tree[crr] =
                (self.merge_fn)(&self.tree[crr << 1], &self.tree[crr << 1 | 1]);

            crr >>= 1;
        }
    }

    pub fn try_set(
        &mut self,
        index: usize,
        value: T,
    ) -> Result<(), SegtreeAccessError> {
        if index >= self.data_len {
            return Err(SegtreeAccessError::IndexOutOfBounds);
        }

        self.set(index, value);
        Ok(())
    }

    pub fn get(&self, index: usize) -> &T {
        &self.tree[index + self.data_len]
    }

    pub fn try_get(&self, index: usize) -> Result<&T, SegtreeAccessError> {
        if index >= self.data_len {
            Err(SegtreeAccessError::IndexOutOfBounds)
        } else {
            Ok(self.get(index))
        }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        let mut resl = self.default.clone();
        let mut resr = self.default.clone();

        let mut l = l + self.data_len;
        let mut r = r + self.data_len;

        while l < r {
            if l & 1 == 1 {
                resl = (self.merge_fn)(&resl, &self.tree[l]);
                l += 1;
            }
            if r & 1 == 0 {
                resr = (self.merge_fn)(&self.tree[r], &resr);
                r -= 1;
            }

            l >>= 1;
            r >>= 1;
        }

        if l == r && l > 0 {
            resl = (self.merge_fn)(&resl, &self.tree[l]);
        }

        (self.merge_fn)(&resl, &resr)
    }

    pub fn try_query(
        &self,
        l: usize,
        r: usize,
    ) -> Result<T, SegtreeRangeError> {
        if l > r {
            Err(SegtreeRangeError::InvalidRange)
        } else if r >= self.data_len {
            Err(SegtreeRangeError::RangeOutOfBounds)
        } else if l == r {
            Ok(self.get(l).clone())
        } else {
            Ok(self.query(l, r))
        }
    }
}
