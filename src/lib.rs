#![no_std]

use core::default::Default;

pub const DEFAULT_BUFFER_SIZE: usize = 4096;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Zstr<const N: usize> {
    pub chars: [u8; N],
    pub len: usize,
}

pub fn new() -> Zstr<DEFAULT_BUFFER_SIZE> {
    Zstr { chars: [0; DEFAULT_BUFFER_SIZE], len: 0 }
}

impl<const N: usize> Zstr<N> {
    pub fn new() -> Self {
        Zstr { chars: [0; N], len: 0 }
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        // Convert the byte slice to a string slice
        core::str::from_utf8_mut(&mut self.chars[..self.len]).unwrap()
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// insert c at index
    pub fn insert(&mut self, index: usize, c: char) {
        // iterate while index < self.len
        // push each char to the array
        let mut i = index;
        let mut prev = c;
        while i < self.len {
            let tmp = self.chars[i] as char;
            self.chars[i] = prev as u8;
            prev = tmp;
            i+=1;
        }
        self.chars[i] = prev as u8;
        self.len += 1;
    }

    pub fn insert_str(&mut self, index: usize, s: &str) {
        // iterate while index < s.len
        // push each char to the array
        let mut i = index;
        for c in s.chars() {
            self.insert(i, c);
            i+=1;
        }        
    }

    pub fn remove(&mut self, index: usize) -> char {
        // remove the char at index
        // return the char
        let mut i = index;
        let mut c = ' ';
        while i < self.len {
            c = self.chars[i] as char;
            self.chars[i] = self.chars[i+1];
            i+=1;
        }
        self.len -= 1;
        c
    }

    pub fn pop(&mut self) -> Option<char> {
        // remove the last char
        // return the char
        if self.len > 0 {
            let c = self.chars[self.len-1] as char;
            self.len -= 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn truncate(&mut self, new_len: usize) {
        // truncate the string to new_len
        // if new_len < self.len
        if new_len < self.len {
            self.len = new_len;
        }
    }

    pub fn as_str(&self) -> &str {
        // Convert the byte slice to a string slice
        core::str::from_utf8(&self.chars[..self.len]).unwrap()
    }

    pub fn push(&mut self, c: char) {
        // Convert the char to a byte sequence
        let dst = &mut [0; 4];
        let bytes = c.encode_utf8(dst).as_bytes();
        // Iterate over the bytes and push them to the array
        for &byte in bytes {
            // Check if there is enough space in the array
            if self.len < N {
                // Add the byte to the array
                self.chars[self.len] = byte;
                // Increment the length
                self.len += 1;
            } else {
                panic!("Zstr is full")
            }
        }
    }

    pub fn push_str(&mut self, s: &str) {
        // Iterate over the chars of the string
        for c in s.chars() {
            // Push each char to the array
            self.push(c);
        }
    }

    pub fn from(s: &str) -> Self {
        let mut zstr = Zstr::<N>::new();
        zstr.push_str(s);
        zstr
    }
}

impl<const N: usize> Default for Zstr<N> {
    fn default() -> Self {
        Zstr::<N>::from("")
    }
}

impl<const N: usize> core::fmt::Display for Zstr<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> core::fmt::Debug for Zstr<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#?}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;

    #[test]
    fn test_new() {
        let mut s = Zstr::<64>::new();
        s.push('a');
        s.push('b');
        s.push('c');

        s.insert(1, 'x');
        std::println!("s: {:#?}", s.as_str());
        s.push_str("bc");
        // assert_eq!(s.as_str(), "abc");

        std::println!("s: {}", s);
    }
}
