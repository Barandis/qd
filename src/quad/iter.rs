// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::iter::{Product, Sum};

impl Sum for Quad {
    /// Sums all of the values in an iterator of `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// use std::iter::Sum;
    ///
    /// let expected = qd!(15);
    /// let actual: Quad = vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].into_iter().sum();
    /// assert!(expected == actual);
    /// # }
    /// ```
    fn sum<I>(iter: I) -> Quad
    where
        I: Iterator<Item = Quad>,
    {
        iter.fold(Quad::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Quad> for Quad {
    /// Sums all of the referenced values in an iterator of `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// use std::iter::Sum;
    ///
    /// let expected = qd!(15);
    /// let actual: Quad = vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().sum();
    /// assert!(expected == actual);
    /// # }
    /// ```
    fn sum<I>(iter: I) -> Quad
    where
        I: Iterator<Item = &'a Quad>,
    {
        iter.fold(Quad::ZERO, |a, b| a + *b)
    }
}

impl Product for Quad {
    /// Multiplies all of the values in an iterator of `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// use std::iter::Product;
    ///
    /// let expected = qd!(120);
    /// let actual: Quad = vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].into_iter().product();
    /// assert!(expected == actual);
    /// # }
    /// ```
    fn product<I>(iter: I) -> Quad
    where
        I: Iterator<Item = Quad>,
    {
        iter.fold(Quad::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Quad> for Quad {
    /// Multiples all of the referenced values in an iterator of `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// use std::iter::Product;
    ///
    /// let expected = qd!(120);
    /// let actual: Quad = vec![qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().product();
    /// assert!(expected == actual);
    /// # }
    /// ```
    fn product<I>(iter: I) -> Quad
    where
        I: Iterator<Item = &'a Quad>,
    {
        iter.fold(Quad::ONE, |a, b| a * *b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let actual: Quad = [qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().sum();
        assert_exact!(qd!(15), actual);

        let actual: Quad = vec![Quad::FRAC_PI_2, Quad::FRAC_PI_3, Quad::FRAC_PI_4]
            .into_iter()
            .sum();
        assert_close!(
            qd!("3.403392041388942675001196998552794791213600182656364639389523308"),
            actual
        );

        let actual: Quad = [qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().sum();
        assert_exact!(qd!(15), actual);

        let actual: Quad = vec![Quad::FRAC_PI_2, Quad::FRAC_PI_3, Quad::FRAC_PI_4]
            .iter()
            .sum();
        assert_close!(
            qd!("3.403392041388942675001196998552794791213600182656364639389523308"),
            actual
        );
    }

    #[test]
    fn sum_empty() {
        let actual: Quad = vec![].iter().sum();
        assert_exact!(Quad::ZERO, actual);
    }

    #[test]
    fn sum_inf() {
        let actual: Quad = vec![qd!(1), Quad::INFINITY, qd!(3)].iter().sum();
        assert_exact!(Quad::INFINITY, actual);

        let actual: Quad = vec![qd!(1), Quad::NEG_INFINITY, qd!(3)].iter().sum();
        assert_exact!(Quad::NEG_INFINITY, actual);

        let actual: Quad = vec![Quad::INFINITY, Quad::NEG_INFINITY].iter().sum();
        assert_exact!(Quad::NAN, actual);
    }

    #[test]
    fn sum_nan() {
        let actual: Quad = vec![qd!(1), qd!(2), Quad::NAN].iter().sum();
        assert_exact!(Quad::NAN, actual);
    }

    #[test]
    fn product() {
        let actual: Quad = [qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().product();
        assert_exact!(qd!(120), actual);

        let actual: Quad = vec![Quad::FRAC_PI_2, Quad::FRAC_PI_3, Quad::FRAC_PI_4]
            .into_iter()
            .product();
        assert_close!(
            qd!("1.291928195012492507311513127795891466759387023578546153922689088"),
            actual
        );

        let actual: Quad = [qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].iter().product();
        assert_exact!(qd!(120), actual);

        let actual: Quad = vec![Quad::FRAC_PI_2, Quad::FRAC_PI_3, Quad::FRAC_PI_4]
            .iter()
            .product();
        assert_close!(
            qd!("1.291928195012492507311513127795891466759387023578546153922689088"),
            actual
        );
    }

    #[test]
    fn product_empty() {
        let actual: Quad = vec![].iter().product();
        assert_exact!(Quad::ONE, actual);
    }

    #[test]
    fn product_inf() {
        let actual: Quad = vec![qd!(1), Quad::INFINITY, qd!(3)].iter().product();
        assert_exact!(Quad::INFINITY, actual);

        let actual: Quad = vec![qd!(1), Quad::NEG_INFINITY, qd!(3)].iter().product();
        assert_exact!(Quad::NEG_INFINITY, actual);

        let actual: Quad = vec![Quad::INFINITY, Quad::NEG_INFINITY].iter().product();
        assert_exact!(Quad::NEG_INFINITY, actual);
    }

    #[test]
    fn product_nan() {
        let actual: Quad = vec![qd!(1), qd!(2), Quad::NAN].iter().product();
        assert_exact!(Quad::NAN, actual);
    }
}
