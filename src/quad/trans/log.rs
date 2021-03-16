// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the number.
    ///
    /// If the goal is to calculate the base *e*, base 2, or base 10 logarithms
    /// of `self`, the specialized functions for those purposes([`ln`],
    /// [`log2`], and [`log10`] respectively) will be more efficient.
    ///
    /// [`ln`]: #method.ln
    /// [`log2`]: #method.log2
    /// [`log10`]: #method.log10
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(10).log(7.3);
    /// let expected = qd!("1.158315209978887965104764376269736420106652944692834002126233653");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn log(self, b: f64) -> Quad {
        self.ln() / Quad::from(b).ln()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log() {
        assert_close!(
            qd!("1.174731503667180022671874948332360514453253860423778048991647180"),
            qd!(10).log(7.1)
        );
        assert_close!(
            qd!("4.224809005935378615289228804344351219807607162037233517389353517"),
            qd!(243).log(3.67)
        );
        assert_exact!(Quad::ZERO, qd!(1).log(6.3));
        assert_close!(Quad::ONE, qd!(3.3).log(3.3));
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::NAN, Quad::ZERO.log(9.2));
        assert_exact!(Quad::NAN, Quad::NEG_ZERO.log(1.8));
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.log(7.3));
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.log(7.3));
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.log(3.4));
    }

    #[test]
    fn negative() {
        assert_exact!(Quad::NAN, qd!(-1).log(1.8));
    }
}
