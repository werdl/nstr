use core::ops::{RangeBounds, Index};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec<T, const N: usize> {
    pub items: [T; N],
    pub len: usize,
}

impl <T, const N: usize> core::fmt::Display for Vec<T, N> 
where T: core::fmt::Display + Default + Copy + PartialEq {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut i = 0;
        write!(f, "[")?;
        while i < self.len {
            write!(f, "{}", self.items[i])?;
            i += 1;

            if i < self.len {
                write!(f, ", ")?;
            }
        }
        write!(f, "]");
        Ok(())
    }
}

impl <T, const N: usize> core::fmt::Debug for Vec<T, N> 
where T: core::fmt::Display + Default + Copy + PartialEq {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} - {}", self, self.capacity())
    }
}

impl<T, const N: usize> Vec<T, N> 
where T: Default + Copy + PartialEq {
    pub fn new() -> Self {
        Vec::<T, N> {
            items: [Default::default(); N],
            len: 0,
        }
    }

    /// [`std::vec::Vec::append`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append)
    pub fn append(&mut self, item: T) {
        self.items[self.len] = item;
        self.len += 1;
    }

    /// [`std::vec::Vec::as_mut_ptr`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_ptr)
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.items.as_mut_ptr()
    }

    /// [`std::vec::Vec::as_ptr`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_ptr)
    pub fn as_ptr(&self) -> *const T {
        self.items.as_ptr()
    }

    /// [`std::vec::Vec::as_slice`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice)
    pub fn as_slice(&self) -> &[T] {
        &self.items[..self.len]
    }

    /// [`std::vec::Vec::as_mut_slice`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice)
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.items[..self.len]
    }

    /// [`std::vec::Vec::capacity`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity)
    pub fn capacity(&self) -> usize {
        N
    }

    /// [`std::vec::Vec::clear`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear)
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// [`std::vec::Vec::contains`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.contains)
    pub fn contains(&self, item: &T) -> bool {
        let mut i = 0;
        while i < self.len {
            if self.items[i] == *item {
                return true;
            }
            i += 1;
        }
        false
    }

    /// [`std::vec::Vec::dedup`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup)
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

    /// [`std::vec::Vec::dedup_by`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by)
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

    /// [`std::vec::Vec::dedup_by_key`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by_key)
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

    /// [`std::vec::Vec::drain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain)
    pub fn drain(&mut self) -> core::slice::IterMut<'_, T> {
        self.len = 0;
        self.items.iter_mut()
    }

    /// [`std::vec::Vec::extend_from_slice`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend_from_slice)
    pub fn extend_from_slice(&mut self, other: &[T]) {
        for item in other {
            self.append(*item);
        }
    }

    /// [`std::vec::Vec::extend_from_within`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extend_from_within)
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

    /// [`std::vec::Vec::extract`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extract)
    pub fn extract(&mut self, item: &T) -> Option<T> 
    where T: PartialEq<T> {
        let mut i = 0;
        while i < self.len {
            if self.items[i] == *item {
                return Some(self.remove(i));
            }
            i += 1;
        }
        None
    }

    /// [`std::vec::Vec::extract_if`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.extract_if)
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

    /// [`std::vec::Vec::from_raw_parts`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts)
    pub fn from_raw_parts(items: [T; N], len: usize) -> Self {
        Vec::<T, N> {
            items,
            len,
        }
    }

    /// [`std::vec::Vec::from_raw_parts_in`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts_in)
    pub fn from_raw_parts_in(items: [T; N], len: usize, _capacity: usize) -> Self {
        Vec::<T, N> {
            items,
            len,
        }
    }

    /// [`std::vec::Vec::get`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get)
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.items[index])
        } else {
            None
        }
    }

    /// [`std::vec::Vec::insert`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert)
    pub fn insert(&mut self, index: usize, item: T) {
        self.items.copy_within(index..self.len, index + 1);
        self.items[index] = item;
        self.len += 1;
    }

    /// [`std::vec::Vec::insert_many`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert_many)
    pub fn insert_many(&mut self, index: usize, items: &[T]) {
        self.items.copy_within(index..self.len, index + items.len());
        self.items[index..index + items.len()].copy_from_slice(items);
        self.len += items.len();
    }

    /// [`std::vec::Vec::into_flattened`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_flattened)
    pub fn into_flattened(self) -> [T; N] {
        self.items
    }

    /// [`std::vec::Vec::into_raw_parts`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_raw_parts)
    pub fn into_raw_parts(self) -> ([T; N], usize) {
        (self.items, self.len)
    }

    /// [`std::vec::Vec::into_raw_parts_in`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_raw_parts_in)
    pub fn into_raw_parts_in(self, _capacity: usize) -> ([T; N], usize) {
        (self.items, self.len)
    }

    /// [`std::vec::Vec::is_empty`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// [`std::vec::Vec::len`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len)
    pub fn len(&self) -> usize {
        self.len
    }

    /// [`std::vec::Vec::pop`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop)
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.items[self.len])
        } else {
            None
        }
    }

    /// [`std::vec::Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
    pub fn push(&mut self, item: T) {
        self.append(item);
    }

    /// [`std::vec::Vec::push_within_capacity`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push_within_capacity)
    pub fn push_within_capacity(&mut self, item: T) -> bool {
        if self.len < N {
            self.append(item);
            true
        } else {
            false
        }
    }

    /// [`std::vec::Vec::remove`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove)
    pub fn remove(&mut self, index: usize) -> T {
        let item = self.items[index];
        self.items.copy_within(index + 1..self.len, index);
        self.len -= 1;
        item
    }

    /// [`std::vec::Vec::resize`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.resize)
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

    /// [`std::vec::Vec::resize_with`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.resize_with)
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

    /// [`std::vec::Vec::retain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain)
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

    /// [`std::vec::Vec::retain_mut`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain_mut)
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

    /// [`std::vec::Vec::set_len`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.set_len)
    pub fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }

    /// [`std::vec::Vec::spare_capacity_mut`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.spare_capacity_mut)
    pub fn spare_capacity_mut(&self) -> usize {
        N - self.len
    }

    /// [`std::vec::Vec::split_off`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off)
    pub fn split_off(&mut self, at: usize) -> Vec<T, N> {
        let mut other = Vec::<T, N>::new();
        other.extend_from_within(at..self.len);
        self.len = at;
        other
    }

    /// [`std::vec::Vec::swap_remove`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_remove)
    pub fn swap_remove(&mut self, index: usize) -> T {
        let item = self.items[index];
        self.items[index] = self.items[self.len - 1];
        self.len -= 1;
        item
    }

    /// [`std::vec::Vec::truncate`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate)
    pub fn truncate(&mut self, new_len: usize) {
        self.len = new_len;
    }

    /// [`std::vec::Vec::try_push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_push)
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

    /// [`std::vec::Vec::into_iter`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_iter)
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T, const N: usize> Index<usize> for Vec<T, N> {
    type Output = T;

    /// [`std::vec::Vec::index`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.index)
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T, const N: usize> Index<core::ops::Range<usize>> for Vec<T, N> {
    type Output = [T];

    /// [`std::vec::Vec::index`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.index)
    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        &self.items[index]
    }
}