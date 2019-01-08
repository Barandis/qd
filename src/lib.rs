// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Rust implementation of double-double and quad-double high-precision floating point numbers.
//!
//! The most precise floating-point type in Rust (and most languages) is 64 bits, which gives around
//! 15 decimal digits of precision. This is fine for nearly all applications, but sometimes a little
//! more is needed. (In the author's case, the need for more precision was driven by wanting to zoom
//! further into a Mandelbrot set visualizer that he was/is building.)
//!
//! The choices are limited for higher-precision floating-point numbers.
//!
//! One theoretical possibility is to simply have a type with more bits. IEEE 754 (which describes
//! standards for both floating-point and integer representations on computers) actually defines a
//! 128-bit floating-point type, and there's no reason this couldn't be extended to 256 bits or
//! more. However, floating-point math is done in specialized hardware in the CPU for speed, and
//! there is no common CPU that has a 128-bit FPU. This is why very few languages (really only
//! Fortran and some C/C++ compilers) support 128-bit floating-point numbers, even while many (Rust
//! included) have 128-bit integers.
//!
//! A more realistic possibility is arbitrary precision math, available as libraries in nearly every
//! language and even built into some. Abritrary precision is exactly that, allowing operations on
//! numbers with thousands of digits of precision. The trade-off is that performance is abysmal in
//! most cases. Arbitrary precision numbers are typically stored in a *multiple-digit* format, such
//! as an array of the characters 0-9, and as such the library has to essentially implement math
//! from scratch to work with that format. The performance hit is massive, but for applications that
//! need extraordinary precision, arbitrary precision is the best (and indeed only) option.
//!
//! But there are a lot of applications that could use more precision than what's provided by 64
//! bits, yet don't need more than twice or quadruple that. For these applications, a
//! *multiple-component* format, where high-precision numbers are represented as the unevaluated
//! sums of low-precision numbers, can be very effective. They offer 128 to 256 bits of precision
//! with a hit to performance that is much more manageable. That's where this library comes in.
//!
//! # Double-double and quad-double numbers
//!
//! The numbers provided by this library are double-doubles, represented by two `f64`s, and
//! quad-doubles, represented by four `f64`s. Every effort has been put into making them work as
//! much like `f64`s as possible.
//!
//! The `DoubleDouble` type has 106 bits of significand, meaning about 32 decimal digits, while the
//! `QuadDouble` type has 212 bits (about 64 decimal digits). However, the exponents remain the same
//! as in `f64`, so the range of each type is similar to `f64` (max value of around
//! ~10<sup>308</sup>). These types don't make *bigger* numbers, they make *more precise* numbers.
//!
//! For those who are interested, a paper from MIT called [Library for Double-Double and Quad-Double
//! Arithmetic][1] explains the algorithms for working with these numbers in great detail, and that
//! paper plus their C++ implementation were absolutely invaluable in writing this library.
//!
//! [1]: http://web.mit.edu/tabbott/Public/quaddouble-debian/qd-2.3.4-old/docs/qd.pdf

mod basic;
mod double;

pub mod error;

pub use self::double::DoubleDouble;
