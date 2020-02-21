// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the number times 2<sup>`n`</sup>.
    ///
    /// Though this is not an everyday operation, it is often used in more
    /// advanced mathematical calculations (including several within this
    /// library). Therefore an implementation that is much more efficient than
    /// calculating it through multiplication and [`powi`] is offered despite it
    /// not being part of the `f64` API.
    ///
    /// [`powi`]: #method.powi
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.ldexp(3) == qd!(24)); // 3 * 2^3
    /// # }
    /// ```
    #[inline]
    pub fn ldexp(self, n: i32) -> Quad {
        let factor = 2f64.powi(n);
        Quad(
            self.0 * factor,
            self.1 * factor,
            self.2 * factor,
            self.3 * factor,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldexp() {
        assert_exact!(qd!(48), qd!(3).ldexp(4));
        assert_close!(qd!(0.078_125), qd!(5).ldexp(-6));
        assert_close!(
            qd!("4.216574282663130924562182077780080660863911808152513230508318081e8"),
            Quad::PI.ldexp(27)
        );
        assert_close!(
            qd!("0.00002073884451644169033325414635736589430051610636672942790959905722"),
            Quad::E.ldexp(-17)
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.ldexp(2));
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_ZERO.ldexp(2));
        assert_exact!(Quad::ONE, Quad::ONE.ldexp(0));
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.ldexp(4));
        assert_exact!(Quad::INFINITY, Quad::INFINITY.ldexp(-4));
        assert_exact!(Quad::INFINITY, Quad::INFINITY.ldexp(0));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.ldexp(3));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.ldexp(-3));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.ldexp(0));
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.ldexp(5));
    }
}
