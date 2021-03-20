// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Rust implementation of double-double and quad-double high-precision floating point
//! numbers.
//!
//! The most precise floating-point type in Rust (and most languages) is 64 bits, which
//! gives around 15 decimal digits of precision. This is fine for nearly all applications,
//! but sometimes a little more is needed.
//!
//! The choices are limited for higher-precision floating-point numbers. One choice is to
//! use 128-bit floating-point numbers, but Rust (and most languages outside Fortran and a
//! few C++ flavors) doesn't have them.
//!
//! A second choice is an arbitary-precision library. These are fantastic in that they can
//! do computations in any precision you choose, even into the thousands or millions of
//! digits. Their downside is that internally they use something like character arrays to
//! represent numbers, so they have to essentially re-implement math for that internal
//! representation. This is slow.
//!
//! Fortunately, while a lot of applications need more than the language-provided precision,
//! they don't need as much as arbitrary-precision has to offer. For those cases there is
//! another choice: representing high-precision numbers as unevaluated sums of
//! lower-precision numbers. This choice will give precision in multiples of system-provided
//! number precisions, and while math with these numbers is slower than with regular
//! numbers, it's much faster than arbitrary precision. These sorts of numbers are what this
//! library provides.
//!
//! # Double-double and quad-double numbers
//!
//! The numbers provided by this library are double-doubles, represented by two `f64`s, and
//! quad-doubles, represented by four `f64`s. The names "double-double" and "quad-double"
//! come from IEEE-754 double-precision floating point numbers and are the names used for
//! these numbers the most in literature. Therefore those names are retained even though
//! Rust represents *its* doubles with `f64`. Every effort has been put into making them
//! work as much like `f64`s as possible.
//!
//! The `Double` type (double-double) has 106 bits of significand, meaning about 31 decimal
//! digits, while the `Quad` type (quad-double) has 212 bits (about 63 decimal digits).
//! However, the exponents remain the same as in `f64`, so the range of each type is similar
//! to `f64` (max value of around ~10<sup>308</sup>). These types don't make *bigger*
//! numbers, they make *more precise* numbers.
//!
//! For those who are interested, a paper from MIT called [Library for Double-Double and
//! Quad-Double Arithmetic][1] explains the algorithms for working with these numbers in
//! great detail, and that paper plus their C++ implementation were absolutely invaluable in
//! writing this library.
//!
//! # Using double-double and quad-double numbers
//!
//! `qd` provides a pair of macros, [`dd!`][2] and [`qd!`][3], which can be used to create
//! double-doubles and quad-doubles, respectively. These macros will take any primitive
//! number type (`dd!` cannot take `u128` or `i128`, as there would be a *loss* of precision
//! to turn those into double-doubles) or a string containing a number that can be
//! represented (if the string contains more digits than can be accurately represented by
//! the type, the extra digits will be ignored).
//!
//! Once you have a double-double or a quad-double, you can use it just like you would an
//! `f64`: all of the mathematical operators work on them, the vast majority of methods
//! work, etc. (see the rest of this documentation for the full API). Each type has a full
//! `Display` implementation, meaning that you can use formatting strings with `format!`,
//! `println!` and the like with all of the formatting options that are available with
//! `f64`.
//!
//! It's important to note that double-doubles and quad-doubles are incompatible with each
//! other and with other numbers, short of options to directly convert one to another. In
//! other words, you can't add an `f64` to a `Double` (though you can convert the `f64` to a
//! `Double` and then add them), and you can't multiply a `Quad` by an `i32` (though once
//! again, you can convert the `i32` to a `Quad` and then do it). This is typical of type
//! casting in Rust (you also can't add an `f64` and an `f32` together) and actually makes
//! it less insanity-inducing when reading code with a lot of different number types.
//!
//! # Normalization
//!
//! Since double-doubles and quad-doubles are represented as sums, there is actually an
//! infinite number of ways to represent any of them. For example, 0 could be represented
//! as (0, 0), (1, -1), (π, -π), or any other such pair.
//!
//! This creates a problem if for no other reason than that figuring out what number is
//! equal to whatever other number becomes really hard when an infinite number of pairs all
//! might be equal (there are plenty of other reasons, too). For that reason, we normalize
//! all double-doubles and quad-doubles.
//!
//! Normalizing a number ensures that each component after the first has an absolute value
//! of 0.5 times the lowest-placed unit of the component before it (ULP, unit in the last
//! place) or less. For example, the first component of π is 3.141592653589793. The ULP of
//! this number is 10<sup>-15</sup>, as that final 3 is 15 places after the decimal. The
//! next component must therefore have an absolute value less than or equal to half that, or
//! 5 &times; 10<sup>-16</sup>. Indeed, the second component of π is 1.2246467991473532
//! &times; 10<sup>-16</sup>.
//!
//! Each number's normalized form is unique. The number 0 as a double-double is (0, 0).
//! There is no other pair of components that satisfies the criteria for normalization.
//! Since the form is now unique, comparisons can be made easily, arithmetic can be done
//! efficiently, and generally everything works better.
//! 
//! *Nearly* every function in qd normalizes when necessary. The sole exceptions are
//! [`Double::raw`][4] and [`Quad::raw`][5], which specifically skip normalization and
//! should only be used on numbers that are already known to be normalized.
//!
//! [1]: http://web.mit.edu/tabbott/Public/quaddouble-debian/qd-2.3.4-old/docs/qd.pdf
//! [2]: macros.dd.html
//! [3]: macros.qd.html
//! [4]: struct.Double.html#methods.raw
//! [5]: struct.Quad.html:methods.raw

#![warn(clippy::all)]
#![allow(clippy::needless_doctest_main)]

mod common;
mod double;
mod quad;

pub mod error;

pub use self::double::Double;
pub use self::quad::Quad;
