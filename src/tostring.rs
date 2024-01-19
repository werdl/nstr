use crate::{String, DEFAULT_BUFFER_SIZE};
use core::fmt::{Write, Error};

pub trait ToString {
    /// convert a type to a String
    fn to_string<const N: usize>(&self) -> String<N>;
}

impl<const N: usize> Write for String<N> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let remaining_capacity = N - self.len;
        let bytes_to_write = s.len().min(remaining_capacity);
        let bytes_to_copy = s.as_bytes().len().min(remaining_capacity);
        self.chars[self.len..self.len + bytes_to_copy].copy_from_slice(&s.as_bytes()[..bytes_to_copy]);
        self.len += bytes_to_write;
        if bytes_to_write < s.len() {
            Err(Error)
        } else {
            Ok(())
        }
    }
}

impl<T> ToString for T 
where
    T: core::fmt::Display,
{
    fn to_string<const N: usize>(&self) -> String<N> {
        let mut s = String::<N>::new();
        write!(s, "{}", self).unwrap();
        s
    }
}