#![no_std]

use core::default::Default;

pub const DEFAULT_BUFFER_SIZE: usize = 4096;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String<const N: usize> {
    pub chars: [u8; N],
    pub len: usize,
}

mod methods;

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
