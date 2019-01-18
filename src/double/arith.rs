// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::basic::*;
use crate::double::Double;
use std::f64;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

// #region Addition

impl Double {
    /// Creates a new double-double representing the sum of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::from_add(1.0, 2.0);
    /// assert!(x == dd!(3.0));
    /// # }
    /// ```
    pub fn from_add(a: f64, b: f64) -> Double {
        Double::from(two_sum(a, b))
    }

    #[inline]
    fn add_double(self, other: Double) -> (f64, f64) {
        let (s0, e0) = two_sum(self.0, other.0);
        let (s1, e1) = two_sum(self.1, other.1);
        let (s2, e2) = quick_two_sum(s0, s1 + e0);
        quick_two_sum(s2, e1 + e2)
    }
}

impl Add for Double {
    type Output = Double;

    #[inline]
    fn add(self, other: Double) -> Double {
        Double::from(self.add_double(other))
    }
}

impl<'a> Add<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn add(self, other: &Double) -> Double {
        Double::from(self.add_double(*other))
    }
}

impl<'a> Add<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn add(self, other: Double) -> Double {
        Double::from(self.add_double(other))
    }
}

impl AddAssign for Double {
    #[inline]
    fn add_assign(&mut self, other: Double) {
        let (a, b) = self.add_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl<'a> AddAssign<&'a Double> for Double {
    #[inline]
    fn add_assign(&mut self, other: &Double) {
        let (a, b) = self.add_double(*other);
        self.0 = a;
        self.1 = b;
    }
}

// #endregion

// #region Subtraction

impl Double {
    /// Creates a new double-double representing the difference of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::from_sub(1.0, 2.0);
    /// assert!(x == dd!(-1.0));
    /// # }
    /// ```
    pub fn from_sub(a: f64, b: f64) -> Double {
        Double::from(two_diff(a, b))
    }

    #[inline]
    fn sub_double(self, other: Double) -> (f64, f64) {
        let (s0, e0) = two_diff(self.0, other.0);
        let (s1, e1) = two_diff(self.1, other.1);
        let (s2, e2) = quick_two_sum(s0, s1 + e0);
        quick_two_sum(s2, e1 + e2)
    }
}

impl Sub for Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: Double) -> Double {
        Double::from(self.sub_double(other))
    }
}

impl<'a> Sub<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: &Double) -> Double {
        Double::from(self.sub_double(*other))
    }
}

impl<'a> Sub<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: Double) -> Double {
        Double::from(self.sub_double(other))
    }
}

impl SubAssign for Double {
    #[inline]
    fn sub_assign(&mut self, other: Double) {
        let (a, b) = self.sub_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl<'a> SubAssign<&'a Double> for Double {
    #[inline]
    fn sub_assign(&mut self, other: &Double) {
        let (a, b) = self.sub_double(*other);
        self.0 = a;
        self.1 = b;
    }
}

impl Neg for Double {
    type Output = Double;

    fn neg(self) -> Double {
        Double(-self.0, -self.1)
    }
}

// #endregion

// #region Multiplication

impl Double {
    /// Creates a new double-double representing the product of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::from_mul(1.0, 2.0);
    /// assert!(x == dd!(2.0));
    /// # }
    /// ```
    pub fn from_mul(a: f64, b: f64) -> Double {
        Double::from(two_prod(a, b))
    }

    #[inline]
    fn mul_double(self, other: Double) -> (f64, f64) {
        let (p, e) = two_prod(self.0, other.0);
        quick_two_sum(p, e + self.0 * other.1 + self.1 * other.0)
    }
}

impl Mul for Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: Double) -> Double {
        Double::from(self.mul_double(other))
    }
}

impl<'a> Mul<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: &Double) -> Double {
        Double::from(self.mul_double(*other))
    }
}

impl<'a> Mul<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: Double) -> Double {
        Double::from(self.mul_double(other))
    }
}

impl MulAssign for Double {
    #[inline]
    fn mul_assign(&mut self, other: Double) {
        let (a, b) = self.mul_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl<'a> MulAssign<&'a Double> for Double {
    #[inline]
    fn mul_assign(&mut self, other: &Double) {
        let (a, b) = self.mul_double(*other);
        self.0 = a;
        self.1 = b;
    }
}

// #endregion

// #region Division

// Helper function needed to avoid the only place in this arithmetic where Double::from must be
// called on a non-tuple, non-integer number. With the current parsing of floats, calling
// Double::from this way in the basic arithmetic would cause a stack overflow.
#[inline]
fn mul_f64(a: Double, b: f64) -> Double {
    let (p, e) = two_prod(a.0, b);
    Double::from(quick_two_sum(p, e + a.1 * b))
}

impl Double {
    /// Creates a new double-double representing the quotient of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::from_div(1.0, 2.0);
    /// assert!(x == dd!(0.5));
    /// # }
    /// ```
    pub fn from_div(a: f64, b: f64) -> Double {
        if b == 0.0 {
            if a == 0.0 {
                Double::NAN
            } else if a.is_sign_negative() == b.is_sign_positive() {
                Double::NEG_INFINITY
            } else {
                Double::INFINITY
            }
        } else {
            let q1 = a / b;

            let (p1, p2) = two_prod(q1, b);
            let (s, e) = two_diff(a, p1);

            let q2 = (s + e - p2) / b;
            Double::from(quick_two_sum(q1, q2))
        }
    }

    #[inline]
    fn div_double(self, other: Double) -> (f64, f64) {
        if other.is_zero() {
            if self.is_zero() {
                (f64::NAN, f64::NAN)
            } else if self.is_sign_negative() == other.is_sign_positive() {
                (f64::NEG_INFINITY, f64::NEG_INFINITY)
            } else {
                (f64::INFINITY, f64::INFINITY)
            }
        } else {
            let q1 = self.0 / other.0;
            let mut r = self - mul_f64(other, q1);

            let q2 = r.0 / other.0;
            r -= mul_f64(other, q2);

            let q3 = r.0 / other.0;
            renorm3(q1, q2, q3)
        }
    }

    /// Calculates the reciprocal of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let r = Double::from(2.0).recip();
    /// assert!(r == dd!(0.5));
    /// # }
    /// ```
    #[inline]
    pub fn recip(self) -> Double {
        Double::ONE / self
    }
}

impl Div for Double {
    type Output = Double;

    #[inline]
    fn div(self, other: Double) -> Double {
        Double::from(self.div_double(other))
    }
}

impl<'a> Div<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn div(self, other: &Double) -> Double {
        Double::from(self.div_double(*other))
    }
}

impl<'a> Div<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn div(self, other: Double) -> Double {
        Double::from(self.div_double(other))
    }
}

impl DivAssign for Double {
    #[inline]
    fn div_assign(&mut self, other: Double) {
        let (a, b) = self.div_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl<'a> DivAssign<&'a Double> for Double {
    #[inline]
    fn div_assign(&mut self, other: &Double) {
        let (a, b) = self.div_double(*other);
        self.0 = a;
        self.1 = b;
    }
}

// #endregion

// #region Mod

impl Rem for Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: Double) -> Double {
        let n = (self / other).trunc();
        self - other * n
    }
}

impl<'a> Rem<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: &Double) -> Double {
        let n = (self / *other).trunc();
        self - *other * n
    }
}

impl<'a> Rem<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: Double) -> Double {
        let n = (self / other).trunc();
        self - other * n
    }
}

impl RemAssign for Double {
    #[inline]
    fn rem_assign(&mut self, other: Double) {
        let a = *self % other;
        self.0 = a.0;
        self.1 = a.1;
    }
}

impl<'a> RemAssign<&'a Double> for Double {
    #[inline]
    fn rem_assign(&mut self, other: &Double) {
        let a = *self % *other;
        self.0 = a.0;
        self.1 = a.1;
    }
}

// #endregion

// #region Iterator-related implementations

impl Sum for Double {
    fn sum<I>(iter: I) -> Double
    where
        I: Iterator<Item = Double>,
    {
        iter.fold(Double::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Double> for Double {
    fn sum<I>(iter: I) -> Double
    where
        I: Iterator<Item = &'a Double>,
    {
        iter.fold(Double::ZERO, |a, b| a + *b)
    }
}

impl Product for Double {
    fn product<I>(iter: I) -> Double
    where
        I: Iterator<Item = Double>,
    {
        iter.fold(Double::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Double> for Double {
    fn product<I>(iter: I) -> Double
    where
        I: Iterator<Item = &'a Double>,
    {
        iter.fold(Double::ONE, |a, b| a * *b)
    }
}

// #endregion

// #region Tests

// Tests are all to be done with these types of numbers:
//
// 1 Whole numbers (integers with .0)
// 2 Representable numbers (numbers with even binary fractions)
// 3 Unrepresentable numbers
// 4 High-precision whole numbers
// 5 High-precision representable numbers
// 6 High-precision unrepresentable numbers
// 7 Exponentials with whole numbers
// 8 Exponentials with representable numbers
// 9 Exponentials with unrepresentable numbers
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_close {
        ($expected:expr, $actual:expr $(,)*) => {
            assert_precision!($expected, $actual, 30);
        };
    }

    #[test]
    fn add_whole() {
        assert_close!(27.0, Double::from(13) + Double::from(14));
        assert_close!(
            "9999999999999999999999999".parse::<Double>().unwrap(),
            "1357913579135791357913579".parse::<Double>().unwrap()
                + "8642086420864208642086420".parse::<Double>().unwrap()
        );
        assert_close!(
            Double::from("9999999999999999999999999"),
            Double::from("1357913579135791357913579") + Double::from("8642086420864208642086420"),
        );
        assert_close!(
            dd!("9999999999999999999999999"),
            dd!("1357913579135791357913579") + dd!("8642086420864208642086420")
        );
        assert_close!(8e10, Double::from(3e10) + Double::from(5e10));
    }

    #[test]
    fn add_repr() {
        assert_close!(11.75, Double::from(6.25) + Double::from(5.5));
        assert_close!(
            "99999999999999999999.75".parse::<Double>().unwrap(),
            "13579135791357913579.25".parse::<Double>().unwrap()
                + "86420864208642086420.5".parse::<Double>().unwrap(),
        );
        assert_close!(
            Double::from(1.175e21),
            Double::from(6.25e20) + Double::from(5.5e20),
        );
    }

    #[test]
    fn add_unrepr() {
        assert_close!(10.5, Double::from(6.3) + Double::from(4.2));
        assert_close!(
            "999999999999999.9999999999".parse::<Double>().unwrap(),
            "135791357913579.1357913579".parse::<Double>().unwrap()
                + "864208642086420.8642086420".parse::<Double>().unwrap(),
        );
        assert_close!(2.214e20, Double::from(1.35e20) + Double::from(8.64e19));
    }

    #[test]
    fn sub_whole() {
        assert_close!(-1.0, Double::from(13) - Double::from(14));
        assert_close!(
            "-7284172841728417284172841".parse::<Double>().unwrap(),
            "1357913579135791357913579".parse::<Double>().unwrap()
                - "8642086420864208642086420".parse::<Double>().unwrap(),
        );
        assert_close!(-2e10, Double::from(3e10) - Double::from(5e10));
    }

    #[test]
    fn sub_repr() {
        assert_close!(0.75, Double::from(6.25) - Double::from(5.5));
        assert_close!(
            "-72841728417284172840.75".parse::<Double>().unwrap(),
            "13579135791357913579.5".parse::<Double>().unwrap()
                - "86420864208642086420.25".parse::<Double>().unwrap(),
        );
        assert_close!(7.5e19, Double::from(6.25e20) - Double::from(5.5e20));
    }

    #[test]
    fn sub_unrepr() {
        assert_close!(2.1, Double::from(6.3) - Double::from(4.2));
        assert_close!(
            "-728417284172841.7284172841".parse::<Double>().unwrap(),
            "135791357913579.1357913579".parse::<Double>().unwrap()
                - "864208642086420.8642086420".parse::<Double>().unwrap(),
        );
        assert_close!(4.86e19, Double::from(1.35e20) - Double::from(8.64e19));
    }

    #[test]
    fn mul_whole() {
        assert_close!(182.0, Double::from(13) * Double::from(14));
        assert_close!(
            "117352065029565150100609497180".parse::<Double>().unwrap(),
            Double::from(135791357913579.0) * Double::from(864208642086420.0),
        );
        assert_close!(
            dd!("117352065029565150100609497180"),
            dd!(135791357913579.0) * dd!(864208642086420.0),
        );
        assert_close!(1.5e21, Double::from(3e10) * Double::from(5e10));
    }

    #[test]
    fn mul_repr() {
        assert_close!(34.375, Double::from(6.25) * Double::from(5.5));
        assert_close!(
            "117352065029565616152770018784.875"
                .parse::<Double>()
                .unwrap(),
            "135791357913579.5".parse::<Double>().unwrap()
                * "864208642086420.25".parse::<Double>().unwrap(),
        );
        assert_close!("3.4375e41", Double::from(6.25e20) * Double::from(5.5e20));
    }

    #[test]
    fn mul_unrepr() {
        assert_close!(26.46, Double::from(6.3) * Double::from(4.2));
        assert_close!(
            "1.173520650295653792283007740926518e29"
                .parse::<Double>()
                .unwrap(),
            "135791357913579.13".parse::<Double>().unwrap()
                * "864208642086420.86".parse::<Double>().unwrap(),
        );
        assert_close!(1.1664e20, Double::from(1.35e10) * Double::from(8.64e9));
    }

    #[test]
    fn div_whole() {
        assert_close!(7.0, Double::from(14) / Double::from(2));
        assert_close!(
            "43210432104321043210".parse::<Double>().unwrap(),
            "86420864208642086420".parse::<Double>().unwrap() / Double::from(2),
        );
        assert_close!(2.0, Double::from(1e11) / Double::from(5e10));
    }

    #[test]
    fn div_repr() {
        assert_close!(Double::from(14.5) / Double::from(2), Double::from(7.25));
        assert_close!(
            "43210432104321043210.25".parse::<Double>().unwrap(),
            "86420864208642086420.5".parse::<Double>().unwrap() / Double::from(2),
        );
        assert_close!(2.0, Double::from(1e11) / Double::from(5e10));
    }

    #[test]
    fn div_unrepr() {
        assert_close!(6.5, Double::from(14.3) / Double::from(2.2));
        assert_close!(
            "43210432104321043210.3".parse::<Double>().unwrap(),
            "86420864208642086420.6".parse::<Double>().unwrap() / Double::from(2),
        );
        assert_close!(2.5, Double::from(1.3e11) / Double::from(5.2e10));
    }

    #[test]
    fn sum() {
        assert_eq!(
            dd!(15),
            [dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().sum()
        );
    }

    #[test]
    fn product() {
        assert_eq!(
            dd!(120),
            [dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().product()
        );
    }
}

// #endregion
