// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the least non-negative remainder of $x \div y$, where $x$ is `self` and
    /// $y$ is the argument. This is $x \mod y$ using the Euclidean definition of the modulo
    /// operation.
    ///
    /// In principle, the returned value $r$ will satisfy $0 \le r < |y|$; it will be
    /// non-negative and less than the absolute value of the argument. However, in extreme
    /// cases, $r$ may equal $|y|$ because of floating-point rounding error.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(21);
    /// let y = dd!(4);
    ///
    /// assert!(x.rem_euclid(y) == dd!(1));
    /// assert!((-x).rem_euclid(y) == dd!(3));
    /// assert!(x.rem_euclid(-y) == dd!(1));
    /// assert!((-x).rem_euclid(-y) == dd!(3));
    /// ```
    #[inline]
    pub fn rem_euclid(self, rhs: Double) -> Double {
        let n = rhs.abs();
        self - n * (self / n).floor()
    }

    /// Calculates the quotient in the Euclidean division $x \div y$, where $x$ is `self`
    /// and $y$ is the argument.
    ///
    /// The Euclidean quotient is `x / y` rounded to the integer `n` such that `x >= n * y`.
    /// That makes it the counterpart to [`rem_euclid`] in the same way that regular
    /// division is the counterpart to the `%` operator; the answer `n` that it computes is
    /// the integer such that `x = n * y + x.rem_euclid(y)`.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(21);
    /// let y = dd!(4);
    ///
    /// assert!(x.div_euclid(y) == dd!(5));
    /// assert!((-x).div_euclid(y) == dd!(-6));
    /// assert!(x.div_euclid(-y) == dd!(-5));
    /// assert!((-x).div_euclid(-y) == dd!(6));
    /// ```
    ///
    /// [`rem_euclid`]: #method.rem_euclid
    #[inline]
    pub fn div_euclid(self, rhs: Double) -> Double {
        let q = (self / rhs).trunc();
        if (self % rhs).is_sign_negative() {
            if rhs.is_sign_positive() {
                q - Double::ONE
            } else {
                q + Double::ONE
            }
        } else {
            q
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // rem_euclid tests
    test_all_prec!(
        mod_2pi_e:
            dd!("0.84662165026149600620471182385368251"),
            Double::TAU.rem_euclid(Double::E),
            30;
        mod_2pi_neg_e:
            dd!("0.84662165026149600620471182385368251"),
            Double::TAU.rem_euclid(-Double::E),
            30;
        mod_neg_2pi_e:
            dd!("1.871660178197549229155575647498979"),
            (-Double::TAU).rem_euclid(Double::E),
            31;
        mod_neg_2pi_neg_e:
            dd!("1.871660178197549229155575647498979"),
            (-Double::TAU).rem_euclid(-Double::E),
            31;
    );
    test_all_exact!(
        mod_1_0:
            Double::NAN,
            Double::ONE.rem_euclid(Double::ZERO);
    );

    // div_euclid tests
    test_all_exact!(
        div_2pi_e:
            dd!(2),
            Double::TAU.div_euclid(Double::E);
        div_2pi_neg_e:
            dd!(-2),
            Double::TAU.div_euclid(-Double::E);
        div_neg_2pi_e:
            dd!(-3),
            (-Double::TAU).div_euclid(Double::E);
        div_neg_2pi_neg_e:
            dd!(3),
            (-Double::TAU).div_euclid(-Double::E);
    );
}
