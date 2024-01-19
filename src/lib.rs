/*!
A string library for systems with no global allocator and no standard library. That is to say, everything is allocated on the stack.
Supports the majority of the `std::string::String` API.
Supports most of the core `std::vec::Vec` API, though with notable exceptions.


# Explanation / Motivation
The goal of this library is to provide a type similar to the `std::string::String` type that is included in every Rust program via the prelude. It also aims to provide a `std::vec::Vec` style type, but this feature is not as supported, as there are more methods that require the prescense of a heap.

This library is intended to be used in systems that do not have a global allocator and do not have a standard library, or in libraries that are targeting such systems.

If you have ever used C or C++, you will be familiar with the `char *` type. This type is a pointer to a sequence of characters that is terminated by a null byte (`\0`).

This library is not dissimilar, in that there is a fixed maximum length and the programmer must keep track of the maximum capacity of the string. Though it may feel similar, it is important to note that under the hood, this library is different. `char *` mutability requires a global allocator, whereas this library does not.

One thing we do not have to worry about is null bytes, as we instead keep track of the length of the string, which is handy for easy truncation.

Because of the nature of the way this library must be implemented, it is not possible to be sure that any of the functions won't panic. If you want to do this, just ensure you keep track of the capacity of the string yourself, then place guards in your code before you call any method such as `push` or `push_str` that may cause the string to grow beyond its capacity.

Please note that this library is not a complete drop-in replacement for `std::string::String`, but in most cases it should be sufficient. It implements the majority of the API, but there are some things that are not possible to implement without a global allocator, or some iterators such as `Drain` that are not in the `core` library.

It is also worth noting that in some places, the function signatures differ slightly in a couple of places, as we do not have the ability to use the `std` libary.

Please note that this library lacks the optimizations that the `std` library has, and is not intended to be used in performance critical code.  

# Missing APIs
- pattern matching (unstable in std)
- the ability to grow in place (requires a global allocator)

# Supported APIs
- Everything else
- the `ToString` trait (implemented for everything which implements `core::fmt::Display`)
*/

#![no_std]

use core::default::Default;

pub const DEFAULT_BUFFER_SIZE: usize = 4096;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String<const N: usize> {
    pub chars: [u8; N],
    pub len: usize,
}

mod methods;
mod tostring;
mod vec;

pub use tostring::ToString;
pub use methods::*;
pub use vec::Vec;

/// Macro for creating a `Vec`, similar to the `vec!` macro in the `std` library
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::<u8, DEFAULT_BUFFER_SIZE>::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// Macro for creating a `String`, similar to the `string!` macro in the `std` library
macro_rules! string {
    ($x:expr) => {
        {
            let mut temp_string = String::<DEFAULT_BUFFER_SIZE>::new();
            temp_string.push_str(stringify!($x));
            temp_string
        }
    };

    ($($x:expr),*) => {
        {
            let mut temp_string = String::<DEFAULT_BUFFER_SIZE>::new();
            $(
                temp_string.push($x);
            )*
            temp_string
        }
    };
}

/// Default implementation of `String`
impl<const N: usize> Default for String<N> {
    fn default() -> Self {
        String::<N>::from("")
    }
}

/// Implementation of `core::fmt::Display` for `String`
impl<const N: usize> core::fmt::Display for String<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Implementation of `core::fmt::Debug` for `String`
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
        s.push('ö');

        s.insert(1, 'x');
        std::println!("s: {:#?}", s.as_str());
        s.push_str("bchahahahaha");

        std::println!("s: {:#?}", s.split("ah"));
        std::println!("42: {:#?}", 42.to_string::<17>());
    }

    #[test]
    fn test_vec() {
        let mut v = vec![1, 2, 3, 4, 5];
        std::println!("v: {}", v);
        v.insert(2, 42);
        std::println!("v: {}", v);
        v.remove(2);
        std::println!("v: {}", v);
    }

    #[test]
    fn test_string() {
        let mut s = string!["abcdef"];
        std::println!("s: {}", s);
        s.insert(2, 'x');
        std::println!("s: {}", s);
        s.remove(2);
        std::println!("s: {}", s);
    }
}
