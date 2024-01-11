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

    /// Creates a new string, with maximum byte length `N`.
    /// 
    /// Note that this doesn't create a string with a maximum length of `N` characters, but rather of `N` bytes. (UTF-8 characters can be multiple bytes long.)
    pub fn new() -> Self {
        String { chars: [0; N], len: 0 }
    }

    /// [`std::string::String::as_mut_str()`](https://doc.rust-lang.org/std/string/struct.String.html#method.as_mut_str)
    pub fn as_mut_str(&mut self) -> &mut str {
        // Convert the byte slice to a string slice
        core::str::from_utf8_mut(&mut self.chars[..self.len]).unwrap()
    }

    /// [`std::string::String::as_str()`](https://doc.rust-lang.org/std/string/struct.String.html#method.as_str)
    pub fn as_str(&self) -> &str {
        // Convert the byte slice to a string slice
        core::str::from_utf8(&self.chars[..self.len]).unwrap()
    }

    /// [`std::string::String::capacity()`](https://doc.rust-lang.org/std/string/struct.String.html#method.capacity)
    pub fn capacity(&self) -> usize {
        N
    }

    pub fn chars(&self) -> core::str::Chars {
        self.as_str().chars()
    }

    /// [`std::string::String::clear()`](https://doc.rust-lang.org/std/string/struct.String.html#method.clear)
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// [`std::string::String::contains()`](https://doc.rust-lang.org/std/string/struct.String.html#method.contains)
    pub fn contains(&self, s: &str) -> bool {
        // iterate over the chars of self
        // check if the chars of self are equal to s
        let mut i = 0;
        let mut j = 0;
        while i < self.len {
            if self.chars[i] == s.as_bytes()[j] {
                j+=1;
                if j == s.len() {
                    return true;
                }
            } else {
                j = 0;
            }
            i+=1;
        }
        false
    }

    /// [`std::string::String::drain()`](https://doc.rust-lang.org/std/string/struct.String.html#method.drain)
    /// Note that this implementation is not the same as the one in the standard library. It does the same, but rather returns the removed chars as a new string. This is because the Drain iterator is not in the `core` library.
    pub fn drain(&mut self, range: core::ops::Range<usize>) -> String<N> {
        // remove the chars in range
        // return the removed chars
        let mut i = range.start;
        let mut j = range.end;
        let mut s = String::<N>::new();
        while i < j {
            s.push(self.chars[i] as char);
            i+=1;
        }
        while j < self.len {
            self.chars[j-(range.end-range.start)] = self.chars[j];
            j+=1;
        }
        self.len -= range.end-range.start;
        s
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

    pub fn repeat(&self, n: usize) -> String<N> {
        // repeat the string n times
        let mut s = String::<N>::new();
        for _ in 0..n {
            s.push_str(self.as_str());
        }
        s
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

    pub fn split(&self, delimiter: char) -> core::str::Split<char> {
        self.as_str().split(delimiter)
    }

    pub fn split_ascii_whitespace(&self) -> core::str::SplitAsciiWhitespace {
        self.as_str().split_ascii_whitespace()
    }

    pub fn split_at(&self, mid: usize) -> (&str, &str) {
        self.as_str().split_at(mid)
    }

    pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str) {
        self.as_mut_str().split_at_mut(mid)
    }

    pub fn split_inclusive(&self, separator: char) -> core::str::SplitInclusive<char> {
        self.as_str().split_inclusive(separator)
    }

    pub fn split_once(&self, separator: char) -> Option<(&str, &str)> {
        self.as_str().split_once(separator)
    }

    pub fn split_terminator(&self, sep: char) -> core::str::SplitTerminator<char> {
        self.as_str().split_terminator(sep)
    }

    pub fn split_whitespace(&self) -> core::str::SplitWhitespace {
        self.as_str().split_whitespace()
    }

    pub fn splitn(&self, n: usize, separator: char) -> core::str::SplitN<char> {
        self.as_str().splitn(n, separator)
    }

    pub fn starts_with(&self, s: &str) -> bool {
        // iterate over the chars of s
        // check if the first chars of self are equal to s
        let mut i = 0;
        let mut j = 0;
        while i < s.len() {
            if self.chars[j] != s.as_bytes()[i] {
                return false;
            }
            i+=1;
            j+=1;
        }
        true
    }

    pub fn to_ascii_lowercase(&self) -> String<N> {
        // convert the string to lowercase
        let mut s = String::<N>::new();
        for c in self.as_str().chars() {
            s.push(c.to_ascii_lowercase());
        }
        s
    }

    pub fn to_ascii_uppercase(&self) -> String<N> {
        // convert the string to uppercase
        let mut s = String::<N>::new();
        for c in self.as_str().chars() {
            s.push(c.to_ascii_uppercase());
        }
        s
    }

    pub fn to_lowercase(&self) -> String<N> {
        // convert the string to lowercase
        let mut s = String::<N>::new();
        for c in self.as_str().chars() {
            s.push(c.to_lowercase().next().unwrap());
        }
        s
    }

    pub fn to_uppercase(&self) -> String<N> {
        // convert the string to uppercase
        let mut s = String::<N>::new();
        for c in self.as_str().chars() {
            s.push(c.to_uppercase().next().unwrap());
        }
        s
    }

    pub fn trim(&self) -> &str {
        // remove the whitespaces at the beginning and the end of the string
        self.as_str().trim()
    }

    pub fn truncate(&mut self, new_len: usize) {
        // truncate the string to new_len
        // if new_len < self.len
        if new_len < self.len {
            self.len = new_len;
        }
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

        std::println!("s: {:?}", s.contains("xabc"));
    }
}
