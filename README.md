# nstr
> no_std, no global allocator strings

A string library for systems with no global allocator and no standard library. That is to say, everything is allocated on the stack.
Supports the majority of the `std::string::String` API.

## Explanation / Motivation
The goal of this library is to provide a type similar to the `std::string::String` type that is included in every Rust program via the prelude.

This library is intended to be used in systems that do not have a global allocator and do not have a standard library, or in libraries that are targeting such systems.

If you have ever used C or C++, you will be familiar with the `char *` type. This type is a pointer to a sequence of characters that is terminated by a null byte (`\0`).

This library is not dissimilar, in that there is a fixed maximum length and the programmer must keep track of the maximum capacity of the string. Though it may feel similar, it is important to note that under the hood, this library is different. `char *` mutability requires a global allocator, whereas this library does not.

One thing we do not have to worry about is null bytes, as we instead keep track of the length of the string, which is handy for easy truncation.

Because of the nature of the way this library must be implemented, it is not possible to be sure that any of the functions won't panic. If you want to do this, just ensure you keep track of the capacity of the string yourself, then place guards in your code before you call any method such as `push` or `push_str` that may cause the string to grow beyond its capacity.

Please note that this library is not a complete drop-in replacement for `std::string::String`, but in most cases it should be sufficient. It implements the majority of the API, but there are some things that are not possible to implement without a global allocator, or some iterators such as `Drain` that are not in the `core` library.

It is also worth noting that in some places, the function signatures differ slightly in a couple of places, as we do not have the ability to use the `std` libary.

Please note that this library lacks the optimizations that the `std` library has, and is not intended to be used in performance critical code.  

## Missing APIs
- pattern matching (unstable in std)
- the ability to grow in place (requires a global allocator)

## Supported APIs
- Everything else
- the `ToString` trait (implemented for everything which implements `core::fmt::Display`)