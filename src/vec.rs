use core::{ops::RangeBounds, fmt::{Error, Write}};

pub struct Vec<T, const N: usize> {
    pub items: [T; N],
    pub len: usize,
}

impl<T, const N: usize> Vec<T, N> 
where T: Default + Copy + PartialEq {
    pub fn new() -> Self {
        Vec::<T, N> {
            items: [Default::default(); N],
            len: 0,
        }
    }

    pub fn append(&mut self, item: T) {
        self.items[self.len] = item;
        self.len += 1;
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.items.as_mut_ptr()
    }

    pub fn as_ptr(&self) -> *const T {
        self.items.as_ptr()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.items[..self.len]
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn dedup(&mut self) {
        let mut i = 0;
        while i < self.len {
            let mut j = i + 1;
            while j < self.len {
                if self.items[i] == self.items[j] {
                    self.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }


    pub fn dedup_by<F>(&mut self, mut same_bucket: F) 
    where F: FnMut(&T, &T) -> bool {
        let mut i = 0;
        while i < self.len {
            let mut j = i + 1;
            while j < self.len {
                if same_bucket(&self.items[i], &self.items[j]) {
                    self.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    pub fn dedup_by_key<F, K>(&mut self, mut key: F) 
    where F: FnMut(&T) -> K,
          K: PartialEq<K> {
        let mut i = 0;
        while i < self.len {
            let mut j = i + 1;
            while j < self.len {
                if key(&self.items[i]) == key(&self.items[j]) {
                    self.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    pub fn drain(&mut self) -> core::slice::IterMut<'_, T> {
        self.len = 0;
        self.items.iter_mut()
    }


    pub fn extend_from_slice(&mut self, other: &[T]) {
        for item in other {
            self.append(*item);
        }
    }

    pub fn extend_from_within(&mut self, range: impl RangeBounds<usize>) {
        let start = match range.start_bound() {
            core::ops::Bound::Included(i) => *i,
            core::ops::Bound::Excluded(i) => *i + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            core::ops::Bound::Included(i) => *i + 1,
            core::ops::Bound::Excluded(i) => *i,
            core::ops::Bound::Unbounded => self.len,
        };
        let len = end - start;
        let mut i = 0;
        while i < len {
            self.append(self.items[start + i]);
            i += 1;
        }
    }

    pub fn extract_if<F>(&mut self, mut f: F) -> Option<T> 
    where F: FnMut(&T) -> bool {
        let mut i = 0;
        while i < self.len {
            if f(&self.items[i]) {
                return Some(self.remove(i));
            }
            i += 1;
        }
        None
    }

    pub fn from_raw_parts(items: [T; N], len: usize) -> Self {
        Vec::<T, N> {
            items,
            len,
        }
    }

    pub fn from_raw_parts_in(items: [T; N], len: usize, _capacity: usize) -> Self {
        Vec::<T, N> {
            items,
            len,
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        self.items.copy_within(index..self.len, index + 1);
        self.items[index] = item;
        self.len += 1;
    }

    pub fn insert_many(&mut self, index: usize, items: &[T]) {
        self.items.copy_within(index..self.len, index + items.len());
        self.items[index..index + items.len()].copy_from_slice(items);
        self.len += items.len();
    }

    pub fn into_flattened(self) -> [T; N] {
        self.items
    }

    pub fn into_raw_parts(self) -> ([T; N], usize) {
        (self.items, self.len)
    }

    pub fn into_raw_parts_in(self, _capacity: usize) -> ([T; N], usize) {
        (self.items, self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new_in(_capacity: usize) -> Self {
        Vec::<T, N> {
            items: [Default::default(); N],
            len: 0,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.items[self.len])
        } else {
            None
        }
    }

    pub fn push(&mut self, item: T) {
        self.append(item);
    }

    pub fn push_within_capacity(&mut self, item: T) -> bool {
        if self.len < N {
            self.append(item);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        let item = self.items[index];
        self.items.copy_within(index + 1..self.len, index);
        self.len -= 1;
        item
    }

    pub fn resize(&mut self, new_len: usize, item: T) {
        if new_len > self.len {
            let mut i = self.len;
            while i < new_len {
                self.append(item);
                i += 1;
            }
        } else {
            self.len = new_len;
        }
    }

    pub fn resize_with<F>(&mut self, new_len: usize, mut f: F) 
    where F: FnMut() -> T {
        if new_len > self.len {
            let mut i = self.len;
            while i < new_len {
                self.append(f());
                i += 1;
            }
        } else {
            self.len = new_len;
        }
    }

    pub fn retain<F>(&mut self, mut f: F) 
    where F: FnMut(&T) -> bool {
        let mut i = 0;
        while i < self.len {
            if !f(&self.items[i]) {
                self.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn retain_mut<F>(&mut self, mut f: F) 
    where F: FnMut(&mut T) -> bool {
        let mut i = 0;
        while i < self.len {
            if !f(&mut self.items[i]) {
                self.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }

    pub fn spare_capacity_mut(&self) -> usize {
        N - self.len
    }

    pub fn split_off(&mut self, at: usize) -> Vec<T, N> {
        let mut other = Vec::<T, N>::new();
        other.extend_from_within(at..self.len);
        self.len = at;
        other
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        let item = self.items[index];
        self.items[index] = self.items[self.len - 1];
        self.len -= 1;
        item
    }

    pub fn truncate(&mut self, new_len: usize) {
        self.len = new_len;
    }

    pub fn try_push(&mut self, item: T) -> Result<(), T> {
        if self.len < N {
            self.append(item);
            Ok(())
        } else {
            Err(item)
        }
    }

}

impl<T, const N: usize> core::ops::Deref for Vec<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.items[..self.len]
    }
}

impl<T, const N: usize> IntoIterator for Vec<T, N> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}