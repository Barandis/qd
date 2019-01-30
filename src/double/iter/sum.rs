// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::iter::Sum;

impl Sum for Double {
    /// Sums all of the values in an iterator.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// use std::iter::Sum;
    ///
    /// let expected = dd!(15);
    /// let actual: Double = vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].into_iter().sum();
    /// assert!(expected == actual);
    /// # }
    /// ```
    fn sum<I>(iter: I) -> Double
    where
        I: Iterator<Item = Double>,
    {
        iter.fold(Double::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Double> for Double {
    /// Sums all of the referenced values in an iterator.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// use std::iter::Sum;
    ///
    /// let expected = dd!(15);
    /// let actual: Double = vec![dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().sum();
    /// assert!(expected == actual);
    /// # }
    /// ```
    fn sum<I>(iter: I) -> Double
    where
        I: Iterator<Item = &'a Double>,
    {
        iter.fold(Double::ZERO, |a, b| a + *b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let actual: Double = [dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].into_iter().sum();
        assert_exact!(dd!(15), actual);

        let actual: Double = vec![Double::FRAC_PI_2, Double::FRAC_PI_3, Double::FRAC_PI_4]
            .into_iter()
            .sum();
        assert_close!(dd!("3.4033920413889426750011969985528"), actual);

        let actual: Double = [dd!(1), dd!(2), dd!(3), dd!(4), dd!(5)].iter().sum();
        assert_exact!(dd!(15), actual);

        let actual: Double = vec![Double::FRAC_PI_2, Double::FRAC_PI_3, Double::FRAC_PI_4]
            .iter()
            .sum();
        assert_close!(dd!("3.4033920413889426750011969985528"), actual);
    }

    #[test]
    fn special() {
        let actual: Double = vec![].iter().sum();
        assert_exact!(Double::ZERO, actual);

        let actual: Double = vec![dd!(1), dd!(2), Double::NAN].iter().sum();
        assert_exact!(Double::NAN, actual);

        let actual: Double = vec![dd!(1), Double::INFINITY, dd!(3)].iter().sum();
        assert_exact!(Double::INFINITY, actual);

        let actual: Double = vec![dd!(1), Double::NEG_INFINITY, dd!(3)].iter().sum();
        assert_exact!(Double::NEG_INFINITY, actual);

        let actual: Double = vec![Double::INFINITY, Double::NEG_INFINITY].iter().sum();
        assert_exact!(Double::NAN, actual);
    }
}
