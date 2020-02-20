// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Rust implementation of double-double and quad-double high-precision floating
//! point numbers.
//!
//! The most precise floating-point type in Rust (and most languages) is 64
//! bits, which gives around 15 decimal digits of precision. This is fine for
//! nearly all applications, but sometimes a little more is needed.
//!
//! The choices are limited for higher-precision floating-point numbers. One
//! choice is to use 128-bit floating-point numbers, but Rust (and most
//! languages outside Fortran and a few C++ flavors) doesn't have them.
//!
//! A second choice is an arbitary-precision library. These are fantastic in
//! that they can do computations in any precision you choose, even into the
//! thousands or millions of digits. Their downside is that internally they use
//! something like character arrays to represent numbers, so they have to
//! essentially re-implement math for that internal representation. This is
//! slow.
//!
//! Fortunately, while a lot of applications need more than the
//! language-provided precision, they don't need as much as arbitrary-precision
//! has to offer. For those cases there is another choice: representing
//! high-precision numbers as sums of lower-precision numbers. This choice will
//! give precision in multiples of system-provided number precisions, and while
//! math with these numbers is slower than with regular numbers, it's much
//! faster than arbitrary precision. These sorts of numbers are what this
//! library provides.
//!
//! # Double-double and quad-double numbers
//!
//! The numbers provided by this library are double-doubles, represented by two
//! `f64`s, and quad-doubles, represented by four `f64`s. Every effort has been
//! put into making them work as much like `f64`s as possible.
//!
//! The `Double` type has 106 bits of significand, meaning about 31 decimal
//! digits, while the `Quad` type has 212 bits (about 63 decimal digits).
//! However, the exponents remain the same as in `f64`, so the range of each
//! type is similar to `f64` (max value of around ~10<sup>308</sup>). These
//! types don't make *bigger* numbers, they make *more precise* numbers.
//!
//! For those who are interested, a paper from MIT called [Library for
//! Double-Double and Quad-Double Arithmetic][1] explains the algorithms for
//! working with these numbers in great detail, and that paper plus their C++
//! implementation were absolutely invaluable in writing this library.
//!
//! # Using double-double and quad-double numbers
//!
//! `qd` provides a pair of macros, [`dd!`][2] and `qd!`, which can be used to
//! create double-doubles and quad-doubles, respectively. These macros will take
//! any primitive number type (`dd!` cannot take `u128` or `i128`, as there
//! would be a *loss* of precision to turn those into double-doubles) or a
//! string containing a number that can be represented (if the string contains
//! more digits than can be accurately represented by the type, the extra digits
//! will be ignored).
//!
//! Once you have a double-double or a quad-double, you can use it just like you
//! would an `f64`: all of the mathematical operators work on them, the vast
//! majority of methods work, etc. (see the rest of this documentation for the
//! full API). Each type has a full `Display` implementation, meaning that you
//! can use formatting strings with `format!`, `println!` and the like with all
//! of the formatting options that are available with `f64`.
//!
//! It's important to note that double-doubles and quad-doubles are incompatible
//! with each other and with other numbers, short of options to directly convert
//! one to another. In other words, you can't add an `f64` to a `Double` (though
//! you can convert the `f64` to a `Double` and then add them), and you can't
//! multiply a `Quad` by an `i32` (though once again, you can convert the `i32`
//! to a `Quad` and then do it). This is typical of type casting in Rust (you
//! also can't add an `f64` and an `f32` together) and actually makes it less
//! insanity-inducing when reading code with a lot of different number types.
//!
//! [1]: http://web.mit.edu/tabbott/Public/quaddouble-debian/qd-2.3.4-old/docs/qd.pdf
//! [2]: macros.dd.html

mod common;
mod double;
mod quad;

pub mod error;

pub use self::double::Double;
pub use self::quad::Quad;
