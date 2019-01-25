// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the number.
    ///
    /// If the goal is to calculate the base *e*, base 2, or base 10 logarithms of `self`, the
    /// specialized functions for those purposes([`ln`], [`log2`], and [`log10`] respectively) will
    /// be more efficient.
    ///
    /// [`ln`]: #method.ln
    /// [`log2`]: #method.log2
    /// [`log10`]: #method.log10
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(10).log(7.0);
    /// let expected = dd!("1.18329466245493832681792856164686");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log(self, b: f64) -> Double {
        self.ln() / Double::from(b).ln()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_close!(
            dd!("1.174731503667180022671874948332360514453253860423778048991647180"),
            dd!(10).log(7.1)
        );
        assert_close!(
            dd!("4.224809005935378615289228804344351219807607162037233517389353517"),
            dd!(243).log(3.67)
        );
    }

    #[test]
    fn special() {
        assert_exact!(Double::ZERO, dd!(1).log(6.3));
        assert_exact!(Double::NAN, dd!(0).log(9.2));
        assert_exact!(Double::NAN, dd!(-1).log(1.8));
        assert_close!(Double::ONE, dd!(3.3).log(3.3));
    }
}
