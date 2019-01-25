// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::ops::Neg;

impl Neg for Double {
    type Output = Double;

    fn neg(self) -> Double {
        Double(-self.0, -self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_close!(dd!("-3.1415926535897932384626433832795"), -Double::PI);
        assert_close!(dd!("-2.7182818284590452353602874713527"), -Double::E);
    }

    #[test]
    fn special() {
        assert_exact!(Double::NAN, -Double::NAN);
        assert_exact!(Double::NEG_INFINITY, -Double::INFINITY);
        assert_exact!(Double::INFINITY, -Double::NEG_INFINITY);
        assert_exact!(Double::NEG_ZERO, -Double::ZERO);
        assert_exact!(Double::ZERO, -Double::NEG_ZERO);
    }
}
