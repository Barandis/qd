// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the arccosine of the number. The return value is in the range [0, π] for any number
    /// in the range [-1, 1]. Otherwise the return value is `NaN`.
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
    fn calc() {
        assert_close!(dd!("1.0471975511965977461542144610932"), dd!(0.5).acos());
    }

    #[test]
    fn edge() {
        assert_exact!(Double::NAN, dd!(1.5).acos());
        assert_exact!(Double::NAN, dd!(-1.5).acos());
        assert_exact!(Double::ZERO, dd!(1).acos());
        assert_close!(Double::PI, dd!(-1).acos());
    }
}
