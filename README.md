# qd

High-precision floating point numbers for Rust

---

qd is a Rust implementation of double-doubles and quad-doubles. These are numbers that are represented as unevaluated sums of multiple `f64` numbers, allowing double or quadruple accuracy over their unadorned `f64` counterparts. An effort is being made to make their use as close to that of the `f64` as possible.

---

The basic functionality of this project is largely finished, but only just. It is considered alpha quality software and is not ready for use in production systems unless you enjoy excitement.

This project languished for a time while I was handling other things, but it is back under active development, including the recent completion of the quad-double portion.

[Current API documentation is available here](https://barandis.github.com/qd/qd/index.html).
