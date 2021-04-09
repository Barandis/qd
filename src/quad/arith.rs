// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
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
    /// # use qd::qd;
    /// let x = qd!(21);
    /// let y = qd!(4);
    ///
    /// assert!(x.rem_euclid(y) == qd!(1));
    /// assert!((-x).rem_euclid(y) == qd!(3));
    /// assert!(x.rem_euclid(-y) == qd!(1));
    /// assert!((-x).rem_euclid(-y) == qd!(3));
    /// ```
    #[inline]
    pub fn rem_euclid(self, rhs: Quad) -> Quad {
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
    /// # use qd::qd;
    /// let x = qd!(21);
    /// let y = qd!(4);
    ///
    /// assert!(x.div_euclid(y) == qd!(5));
    /// assert!((-x).div_euclid(y) == qd!(-6));
    /// assert!(x.div_euclid(-y) == qd!(-5));
    /// assert!((-x).div_euclid(-y) == qd!(6));
    /// ```
    ///
    /// [`rem_euclid`]: #method.rem_euclid
    #[inline]
    pub fn div_euclid(self, rhs: Quad) -> Quad {
        let q = (self / rhs).trunc();
        if (self % rhs).is_sign_negative() {
            if rhs.is_sign_positive() {
                q - Quad::ONE
            } else {
                q + Quad::ONE
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
    test_all_near!(
        mod_2pi_e:
            qd!("0.84662165026149600620471182385368077287984461135029249201595392916595"),
            Quad::TAU.rem_euclid(Quad::E);
        mod_2pi_neg_e:
            qd!("0.84662165026149600620471182385368077287984461135029249201595392916595"),
            Quad::TAU.rem_euclid(-Quad::E);
        mod_neg_2pi_e:
            qd!("1.871660178197549229155575647498981724877402482349667082951013698558"),
            (-Quad::TAU).rem_euclid(Quad::E);
        mod_neg_2pi_neg_e:
            qd!("1.871660178197549229155575647498981724877402482349667082951013698558"),
            (-Quad::TAU).rem_euclid(-Quad::E);
    );
    test_all_exact!(
        mod_1_0:
            Quad::NAN,
            Quad::ONE.rem_euclid(Quad::ZERO);
    );

    // div_euclid tests
    test_all_exact!(
        div_2pi_e:
            qd!(2),
            Quad::TAU.div_euclid(Quad::E);
        div_2pi_neg_e:
            qd!(-2),
            Quad::TAU.div_euclid(-Quad::E);
        div_neg_2pi_e:
            qd!(-3),
            (-Quad::TAU).div_euclid(Quad::E);
        div_neg_2pi_neg_e:
            qd!(3),
            (-Quad::TAU).div_euclid(-Quad::E);
    );
}
