use crate::String;

impl<const N: usize> core::ops::Deref for String<N> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
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
    

    /// [`std::string::String::chars()`](https://doc.rust-lang.org/std/string/struct.String.html#method.chars)
    pub fn chars(&self) -> core::str::Chars {
        self.as_str().chars()
    }

    /// [`std::string::String::clear()`](https://doc.rust-lang.org/std/string/struct.String.html#method.clear)
    pub fn clear(&mut self) {
        self.len = 0;
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


    /// [`std::string::String::extend_from_within()`](https://doc.rust-lang.org/std/string/struct.String.html#method.extend_from_within)
    pub fn extend_from_within(&mut self, range: core::ops::Range<usize>) {
        // extend the string with the chars in range
        let mut i = range.start;
        while i < range.end {
            self.push(self.chars[i] as char);
            i+=1;
        }
    }

    
    /// [`std::string::String::from()`](https://doc.rust-lang.org/std/string/struct.String.html#method.from)
    pub fn from(s: &str) -> Self {
        let mut str = String::<N>::new();
        str.push_str(s);
        str
    }

    /// [`std::string::String::get()`](https://doc.rust-lang.org/std/string/struct.String.html#method.get)
    pub fn get<I>(&self, index: I) -> Option<&str>
    where
        I: core::slice::SliceIndex<[u8], Output = [u8]>,
    {
        self.chars.get(index).map(|slice| core::str::from_utf8(slice)).transpose().unwrap()
    }

    /// [`std::string::String::get_mut()`](https://doc.rust-lang.org/std/string/struct.String.html#method.get_mut)
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut str>
    where
        I: core::slice::SliceIndex<[u8], Output = [u8]>,
    {
        self.chars.get_mut(index).map(|slice| core::str::from_utf8_mut(slice)).transpose().unwrap()
    }

    /// [`std::string::String::get_unchecked()`](https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked)
    pub fn get_unchecked<I>(&self, index: I) -> &str
    where
        I: core::slice::SliceIndex<[u8], Output = [u8]>,
    {
        core::str::from_utf8(self.chars.get(index).unwrap()).unwrap()
    }

    /// [`std::string::String::get_unchecked_mut()`](https://doc.rust-lang.org/std/string/struct.String.html#method.get_unchecked_mut)
    pub fn get_unchecked_mut<I>(&mut self, index: I) -> &mut str
    where
        I: core::slice::SliceIndex<[u8], Output = [u8]>,
    {
        core::str::from_utf8_mut(self.chars.get_mut(index).unwrap()).unwrap()
    }

    /// [`std::string::String::insert()`](https://doc.rust-lang.org/std/string/struct.String.html#method.insert)
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

    /// [`std::string::String::insert_str()`](https://doc.rust-lang.org/std/string/struct.String.html#method.insert_str)
    pub fn insert_str(&mut self, index: usize, s: &str) {
        // iterate while index < s.len
        // push each char to the array
        let mut i = index;
        for c in s.chars() {
            self.insert(i, c);
            i+=1;
        }        
    }

    /// [`std::string::String::lines()`](https://doc.rust-lang.org/std/string/struct.String.html#method.lines)
    pub fn lines(&self) -> core::str::Lines {
        self.as_str().lines()
    }

    /// [`std::string::String::make_ascii_lowercase()`](https://doc.rust-lang.org/std/string/struct.String.html#method.make_ascii_lowercase)
    pub fn make_ascii_lowercase(&mut self) {
        // convert the string to ascii
        for c in self.clone().as_str().chars() {
            self.push(c.to_ascii_lowercase());
        }
    }

    /// [`std::string::String::make_ascii_uppercase()`](https://doc.rust-lang.org/std/string/struct.String.html#method.make_ascii_uppercase)
    pub fn make_ascii_uppercase(&mut self) {
        // convert the string to ascii
        for c in self.clone().as_str().chars() {
            self.push(c.to_ascii_uppercase());
        }
    }


    /// [`std::string::String::pop()`](https://doc.rust-lang.org/std/string/struct.String.html#method.pop)
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

    /// [`std::string::String::push()`](https://doc.rust-lang.org/std/string/struct.String.html#method.push)
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

    /// [`std::string::String::push_str()`](https://doc.rust-lang.org/std/string/struct.String.html#method.push_str)
    pub fn push_str(&mut self, s: &str) {
        // Iterate over the chars of the string
        for c in s.chars() {
            // Push each char to the array
            self.push(c);
        }
    }

    /// [`std::string::String::remove()`](https://doc.rust-lang.org/std/string/struct.String.html#method.remove)
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

    /// [`std::string::String::repeat()`](https://doc.rust-lang.org/std/string/struct.String.html#method.repeat)
    pub fn repeat(&self, n: usize) -> String<N> {
        // repeat the string n times
        let mut s = String::<N>::new();
        for _ in 0..n {
            s.push_str(self.as_str());
        }
        s
    }

    /// [`std::string::String::replace()`](https://doc.rust-lang.org/std/string/struct.String.html#method.replace)
    pub fn replace(&self, from: &str, to: &str) -> String<N> {
        // iterate over the chars of self
        // check if the chars of self are equal to from
        // if yes, push to to the string
        let mut i = 0;
        let mut j = 0;
        let mut s = String::<N>::new();
        while i < self.len {
            if self.chars[i] == from.as_bytes()[j] {
                j+=1;
                if j == from.len() {
                    s.push_str(to);
                    j = 0;
                }
            } else {
                j = 0;
            }
            s.push(self.chars[i] as char);
            i+=1;
        }
        s
    }

    /// [`std::string::String::replacen()`](https://doc.rust-lang.org/std/string/struct.String.html#method.replacen)
    pub fn replacen(&self, from: &str, to: &str, n: usize) -> String<N> {
        // iterate over the chars of self
        // check if the chars of self are equal to from
        // if yes, push to to the string
        let mut i = 0;
        let mut j = 0;
        let mut s = String::<N>::new();
        let mut count = 0;
        while i < self.len {
            if self.chars[i] == from.as_bytes()[j] {
                j+=1;
                if j == from.len() {
                    s.push_str(to);
                    j = 0;
                    count+=1;
                    if count == n {
                        break;
                    }
                }
            } else {
                j = 0;
            }
            s.push(self.chars[i] as char);
            i+=1;
        }
        s
    }

    /// [`std::string::String::retain()`](https://doc.rust-lang.org/std/string/struct.String.html#method.retain)
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(char) -> bool,
    {
        // iterate over the chars of self
        // check if the char should be removed
        let mut i = 0;
        while i < self.len {
            if !f(self.chars[i] as char) {
                self.remove(i);
            } else {
                i+=1;
            }
        }
    }

    /// [`std::string::String::rfind()`](https://doc.rust-lang.org/std/string/struct.String.html#method.rfind)
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

    /// [`std::string::String::rmatch_indices()`](https://doc.rust-lang.org/std/string/struct.String.html#method.rmatch_indices)
    pub fn rmatch_indices<'a>(&'a self, substring: &'a str) -> core::str::RMatchIndices<'a, &'a str> {
        self.as_str().rmatch_indices(substring)
    }

    /// [`std::string::String::rmatches()`](https://doc.rust-lang.org/std/string/struct.String.html#method.rmatches)
    pub fn rmatches<'a>(&'a self, substring: &'a str) -> core::str::RMatches<'a, &'a str> {
        self.as_str().rmatches(substring)
    }

    /// [`std::string::String::rsplit()`](https://doc.rust-lang.org/std/string/struct.String.html#method.rsplit)
    pub fn rsplit(&self, delimiter: char) -> core::str::RSplit<char> {
        self.as_str().rsplit(delimiter)
    }

    /// [`std::string::String::rsplit_once()`](https://doc.rust-lang.org/std/string/struct.String.html#method.rsplit_once)
    pub fn rsplit_once(&self, separator: char) -> Option<(&str, &str)> {
        self.as_str().rsplit_once(separator)
    }

    /// [`std::string::String::rsplit_terminator()`](https://doc.rust-lang.org/std/string/struct.String.html#method.rsplit_terminator)
    pub fn rsplit_terminator(&self, sep: char) -> core::str::RSplitTerminator<char> {
        self.as_str().rsplit_terminator(sep)
    }

    /// [`std::string::String::rsplitn()`](https://doc.rust-lang.org/std/string/struct.String.html#method.rsplitn)
    pub fn rsplitn(&self, n: usize, separator: char) -> core::str::RSplitN<char> {
        self.as_str().rsplitn(n, separator)
    }

    /// [`std::string::String::split()`](https://doc.rust-lang.org/std/string/struct.String.html#method.split)
    pub fn split(&self, delimiter: char) -> core::str::Split<char> {
        self.as_str().split(delimiter)
    }

    /// [`std::string::String::split_ascii_whitespace()`](https://doc.rust-lang.org/std/string/struct.String.html#method.split_ascii_whitespace)
    pub fn split_ascii_whitespace(&self) -> core::str::SplitAsciiWhitespace {
        self.as_str().split_ascii_whitespace()
    }

    /// [`std::string::String::split_at()`](https://doc.rust-lang.org/std/string/struct.String.html#method.split_at)
    pub fn split_at(&self, mid: usize) -> (&str, &str) {
        self.as_str().split_at(mid)
    }

    /// [`std::string::String::split_at_mut()`](https://doc.rust-lang.org/std/string/struct.String.html#method.split_at_mut)
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut str, &mut str) {
        self.as_mut_str().split_at_mut(mid)
    }

    /// [`std::string::String::split_inclusive()`](https://doc.rust-lang.org/std/string/struct.String.html#method.split_inclusive)
    pub fn split_inclusive(&self, separator: char) -> core::str::SplitInclusive<char> {
        self.as_str().split_inclusive(separator)
    }

    /// [`std::string::String::split_once()`](https://doc.rust-lang.org/std/string/struct.String.html#method.split_once)
    pub fn split_once(&self, separator: char) -> Option<(&str, &str)> {
        self.as_str().split_once(separator)
    }

    /// [`std::string::String::split_terminator()`](https://doc.rust-lang.org/std/string/struct.String.html#method.split_terminator)
    pub fn split_terminator(&self, sep: char) -> core::str::SplitTerminator<char> {
        self.as_str().split_terminator(sep)
    }

    /// [`std::string::String::split_whitespace()`](https://doc.rust-lang.org/std/string/struct.String.html#method.split_whitespace)
    pub fn split_whitespace(&self) -> core::str::SplitWhitespace {
        self.as_str().split_whitespace()
    }

    /// [`std::string::String::splitn()`](https://doc.rust-lang.org/std/string/struct.String.html#method.splitn)
    pub fn splitn(&self, n: usize, separator: char) -> core::str::SplitN<char> {
        self.as_str().splitn(n, separator)
    }

    /// [`std::string::String::starts_with()`](https://doc.rust-lang.org/std/string/struct.String.html#method.starts_with)
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

    /// [`std::string::String::strip_prefix()`](https://doc.rust-lang.org/std/string/struct.String.html#method.strip_prefix)
    pub fn strip_prefix(&self, prefix: &str) -> Option<&str> {
        self.as_str().strip_prefix(prefix)
    }

    /// [`std::string::String::strip_suffix()`](https://doc.rust-lang.org/std/string/struct.String.html#method.strip_suffix)
    pub fn strip_suffix(&self, suffix: &str) -> Option<&str> {
        self.as_str().strip_suffix(suffix)
    }

    /// [`std::string::String::to_ascii_lowercase()`](https://doc.rust-lang.org/std/string/struct.String.html#method.to_ascii_lowercase)
    pub fn to_ascii_lowercase(&self) -> String<N> {
        // convert the string to lowercase
        let mut s = String::<N>::new();
        for c in self.as_str().chars() {
            s.push(c.to_ascii_lowercase());
        }
        s
    }

    /// [`std::string::String::to_ascii_uppercase()`](https://doc.rust-lang.org/std/string/struct.String.html#method.to_ascii_uppercase)
    pub fn to_ascii_uppercase(&self) -> String<N> {
        // convert the string to uppercase
        let mut s = String::<N>::new();
        for c in self.as_str().chars() {
            s.push(c.to_ascii_uppercase());
        }
        s
    }

    /// [`std::string::String::to_lowercase()`](https://doc.rust-lang.org/std/string/struct.String.html#method.to_lowercase)
    pub fn to_lowercase(&self) -> String<N> {
        // convert the string to lowercase
        let mut s = String::<N>::new();
        for c in self.as_str().chars() {
            s.push(c.to_lowercase().next().unwrap());
        }
        s
    }

    /// [`std::string::String::to_uppercase()`](https://doc.rust-lang.org/std/string/struct.String.html#to_uppercase.to_mut)
    pub fn to_uppercase(&self) -> String<N> {
        // convert the string to uppercase
        let mut s = String::<N>::new();
        for c in self.as_str().chars() {
            s.push(c.to_uppercase().next().unwrap());
        }
        s
    }

    /// [`std::string::String::trim()`](https://doc.rust-lang.org/std/string/struct.String.html#method.trim)
    pub fn trim(&self) -> &str {
        // remove the whitespaces at the beginning and the end of the string
        self.as_str().trim()
    }

    /// [`std::string::String::trim_end()`](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_end)
    pub fn trim_end(&self) -> &str {
        // remove the whitespaces at the end of the string
        self.as_str().trim_end()
    }

    /// [`std::string::String::trim_start()`](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_start)
    pub fn trim_start(&self) -> &str {
        // remove the whitespaces at the beginning of the string
        self.as_str().trim_start()
    }

    /// [`std::string::String::truncate()`](https://doc.rust-lang.org/std/string/struct.String.html#method.truncate)
    pub fn truncate(&mut self, new_len: usize) {
        // truncate the string to new_len
        // if new_len < self.len
        if new_len < self.len {
            self.len = new_len;
        }
    }
}