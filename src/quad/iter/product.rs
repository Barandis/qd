// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::iter::Product;

impl Product for Quad {
    /// Multiplies all of the values in an iterator.
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
    /// Multiples all of the referenced values in an iterator.
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
    fn basic() {
        let actual: Quad = [qd!(1), qd!(2), qd!(3), qd!(4), qd!(5)].into_iter().product();
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
    fn special() {
        let actual: Quad = vec![].iter().product();
        assert_exact!(Quad::ONE, actual);

        let actual: Quad = vec![qd!(1), qd!(2), Quad::NAN].iter().product();
        assert_exact!(Quad::NAN, actual);

        let actual: Quad = vec![qd!(1), Quad::INFINITY, qd!(3)].iter().product();
        assert_exact!(Quad::INFINITY, actual);

        let actual: Quad = vec![qd!(1), Quad::NEG_INFINITY, qd!(3)].iter().product();
        assert_exact!(Quad::NEG_INFINITY, actual);

        let actual: Quad = vec![Quad::INFINITY, Quad::NEG_INFINITY].iter().product();
        assert_exact!(Quad::NEG_INFINITY, actual);
    }
}
