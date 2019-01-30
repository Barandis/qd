// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::iter::Product;

impl Product for Double {
    /// Multiplies all of the values in an iterator.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// use std::iter::Product;
    ///
    /// let expected = dd!(120);
    /// let actual: Double = vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].into_iter().product();
    /// assert!(expected == actual);
    /// # }
    /// ```
    fn product<I>(iter: I) -> Double
    where
        I: Iterator<Item = Double>,
    {
        iter.fold(Double::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Double> for Double {
    /// Multiples all of the referenced values in an iterator.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// use std::iter::Product;
    ///
    /// let expected = dd!(120);
    /// let actual: Double = vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().product();
    /// assert!(expected == actual);
    /// # }
    /// ```
    fn product<I>(iter: I) -> Double
    where
        I: Iterator<Item = &'a Double>,
    {
        iter.fold(Double::ONE, |a, b| a * *b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let actual: Double = [dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].into_iter().product();
        assert_exact!(dd!(120), actual);

        let actual: Double = vec![Double::FRAC_PI_2, Double::FRAC_PI_3, Double::FRAC_PI_4]
            .into_iter()
            .product();
        assert_close!(dd!("1.2919281950124925073115131277959"), actual);

        let actual: Double = [dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().product();
        assert_exact!(dd!(120), actual);

        let actual: Double = vec![Double::FRAC_PI_2, Double::FRAC_PI_3, Double::FRAC_PI_4]
            .iter()
            .product();
        assert_close!(dd!("1.2919281950124925073115131277959"), actual);
    }

    #[test]
    fn special() {
        let actual: Double = vec![].iter().product();
        assert_exact!(Double::ONE, actual);

        let actual: Double = vec![dd!(1), dd!(2), Double::NAN].iter().product();
        assert_exact!(Double::NAN, actual);

        let actual: Double = vec![dd!(1), Double::INFINITY, dd!(3)].iter().product();
        assert_exact!(Double::INFINITY, actual);

        let actual: Double = vec![dd!(1), Double::NEG_INFINITY, dd!(3)].iter().product();
        assert_exact!(Double::NEG_INFINITY, actual);

        let actual: Double = vec![Double::INFINITY, Double::NEG_INFINITY]
            .iter()
            .product();
        assert_exact!(Double::NEG_INFINITY, actual);
    }
}
