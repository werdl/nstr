#![no_std]

use core::default::Default;

pub const DEFAULT_BUFFER_SIZE: usize = 4096;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String<const N: usize> {
    pub chars: [u8; N],
    pub len: usize,
}

pub fn new() -> String<DEFAULT_BUFFER_SIZE> {
    String { chars: [0; DEFAULT_BUFFER_SIZE], len: 0 }
}

impl<const N: usize> String<N> {
    pub fn new() -> Self {
        String { chars: [0; N], len: 0 }
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        // Convert the byte slice to a string slice
        core::str::from_utf8_mut(&mut self.chars[..self.len]).unwrap()
    }

    pub fn as_str(&self) -> &str {
        // Convert the byte slice to a string slice
        core::str::from_utf8(&self.chars[..self.len]).unwrap()
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn ends_with(&self, s: &str) -> bool {
        // iterate over the chars of s
        // check if the last chars of self are equal to s
        let mut i = 0;
        let mut j = self.len - s.len();
        while i < s.len() {
            if self.chars[j] != s.as_bytes()[i] {
                return false;
            }
            i+=1;
            j+=1;
        }
        true
    }

    pub fn find(&self, s: &str) -> Option<usize> {
        // iterate over the chars of self
        // check if the chars of self are equal to s
        let mut i = 0;
        let mut j = 0;
        while i < self.len {
            if self.chars[i] == s.as_bytes()[j] {
                j+=1;
                if j == s.len() {
                    return Some(i-j+1);
                }
            } else {
                j = 0;
            }
            i+=1;
        }
        None
    }
    
    pub fn from(s: &str) -> Self {
        let mut str = String::<N>::new();
        str.push_str(s);
        str
    }

    pub fn get<I>(&self, index: I) -> Option<&str>
    where
        I: core::slice::SliceIndex<[u8], Output = [u8]>,
    {
        self.chars.get(index).map(|slice| core::str::from_utf8(slice)).transpose().unwrap()
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

    pub fn lines(&self) -> core::str::Lines {
        self.as_str().lines()
    }

    pub fn parse<T>(&self) -> T
    where
        T: core::str::FromStr + core::fmt::Debug,
        <T as core::str::FromStr>::Err: core::fmt::Debug,
    {
        self.as_str().parse::<T>().unwrap()
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
                panic!("String is full")
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

    pub fn rfind(&self, s: &str) -> Option<usize> {
        // iterate over the chars of self
        // check if the chars of self are equal to s
        let mut i = self.len - 1;
        let mut j = s.len() - 1;
        while i > 0 {
            if self.chars[i] == s.as_bytes()[j] {
                j-=1;
                if j == 0 {
                    return Some(i-j+1);
                }
            } else {
                j = s.len() - 1;
            }
            i-=1;
        }
        None
    }
}

impl<const N: usize> Default for String<N> {
    fn default() -> Self {
        String::<N>::from("")
    }
}

impl<const N: usize> core::fmt::Display for String<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> core::fmt::Debug for String<N> {
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
        let mut s = String::<64>::new();
        s.push('a');
        s.push('b');
        s.push('c');

        s.insert(1, 'x');
        std::println!("s: {:#?}", s.as_str());
        s.push_str("bc");
        // assert_eq!(s.as_str(), "abc");

        std::println!("s: {:?}", s.find("LOL"));
    }
}
