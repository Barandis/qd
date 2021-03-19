// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the inverse cosine (cos<sup>-1</sup>) of the `Double`. The domain of this
    /// function is [-1, 1] and the range is [0, π]. Arguments outside of the domain will
    /// result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).acos();
    /// let expected = dd!(0);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    /// 
    /// [`NAN`]: #associatedconstant.NAN
    pub fn acos(self) -> Double {
        if self.abs() > Double::ONE {
            Double::NAN
        } else if self == Double::ONE {
            Double::ZERO
        } else if self == -Double::ONE {
            Double::PI
        } else {
            (Double::ONE - self.sqr()).sqrt().atan2(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acos() {
        assert_close!(dd!("1.0471975511965977461542144610932"), dd!(0.5).acos());
        assert_exact!(Double::ZERO, dd!(1).acos());
        assert_close!(Double::PI, dd!(-1).acos());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.acos());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.acos());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.acos());
    }

    #[test]
    fn out_of_range() {
        assert_exact!(Double::NAN, dd!(1.5).acos());
        assert_exact!(Double::NAN, dd!(-1.5).acos());
    }
}
