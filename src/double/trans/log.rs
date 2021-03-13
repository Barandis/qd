// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the number.
    ///
    /// If the goal is to calculate the base *e*, base 2, or base 10 logarithms of `self`,
    /// the specialized functions for those purposes([`ln`], [`log2`], and [`log10`]
    /// respectively) will be more efficient.
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
    fn log() {
        assert_close!(dd!("1.17473150366718002267187494833236"), dd!(10).log(7.1));
        assert_close!(
            dd!("4.22480900593537861528922880434435"),
            dd!(243).log(3.67)
        );
        assert_exact!(Double::ZERO, dd!(1).log(6.3));
        assert_close!(Double::ONE, dd!(3.3).log(3.3));
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NAN, Double::ZERO.log(9.2));
        assert_exact!(Double::NAN, Double::NEG_ZERO.log(1.8));
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log(7.3));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log(7.3));
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.log(3.4));
    }

    #[test]
    fn negative() {
        assert_exact!(Double::NAN, dd!(-1).log(1.8));
    }
}
