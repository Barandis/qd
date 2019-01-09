// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

// #region Exponential

/// Reciprocals of factorials, rendered as Doubles. These are used in the Taylor series for
/// calculating the exponential function.
const INV_FACT: [Double; 15] = [
    Double(1.6666666666666666e-1, 9.25185853854297e-18),
    Double(4.1666666666666664e-2, 2.3129646346357427e-18),
    Double(8.333333333333333e-3, 1.1564823173178714e-19),
    Double(1.388888888888889e-3, -5.300543954373577e-20),
    Double(1.984126984126984e-4, 1.7209558293420705e-22),
    Double(2.48015873015873e-5, 2.1511947866775882e-23),
    Double(2.7557319223985893e-6, -1.858393274046472e-22),
    Double(2.755731922398589e-7, 2.3767714622250297e-23),
    Double(2.505210838544172e-8, -1.448814070935912e-24),
    Double(2.08767569878681e-9, -1.20734505911326e-25),
    Double(1.6059043836821613e-10, 1.2585294588752098e-26),
    Double(1.1470745597729725e-11, 2.0655512752830745e-28),
    Double(7.647163731819816e-13, 7.03872877733453e-30),
    Double(4.779477332387385e-14, 4.399205485834081e-31),
    Double(2.8114572543455206e-15, 1.6508842730861433e-31),
];

/// Helper function that efficiently multiplies a Double by a power of 2. This is -much-
/// faster than regular multiplication but only works with powers of 2.
#[inline]
fn mul_pwr2(a: Double, b: f64) -> Double {
    Double(a.0 * b, a.1 * b)
}

impl Double {
    /// Computes the exponential function, *e*<sup>self</sup>, in double-double precision.
    pub fn exp(&self) -> Double {
        // Strategy, as gleaned from MIT papers and Wikipedia:
        //
        // The first step is to reduce the size of the exponent by noting that
        //
        //      exp(kr + m * ln(2)) = 2^m * exp(r)^k
        //
        // where m and k are arbitary integers. By choosing m appropriately we can make |kr| <=
        // ln(2) / 2 = 0.347. Then exp(r) is evaluated using a Taylor series, which is actually
        // reasonably easy to figure out for the exponential function:
        //
        //      exp(x) = 1 + x + x^2/2! + x^3/3! + x^4/4! ...
        //
        // Reducing x substantially speeds up the convergence, so we have to use fewer terms to
        // reach the required precision.

        let k = 512.0;
        let inv_k = 1.0 / k;

        // Common cases, including numbers too big or small to be represented with Doubles
        if self.0 <= -709.0 {
            return Double::ZERO;
        }
        if self.0 >= 709.0 {
            return Double::INFINITY;
        }
        if self.is_zero() {
            return Double::ONE;
        }
        if self.is_one() {
            return Double::E;
        }

        let m = (self.0 / Double::LN_2.0 + 0.5).floor();
        let r = mul_pwr2(*self - Double::LN_2 * m, inv_k);

        let mut p = r.square();
        let mut s = r + mul_pwr2(p, 0.5);
        p *= r;
        let mut t = p * INV_FACT[0];
        let mut i = 0;

        loop {
            s += t;
            p *= r;
            i += 1;
            t = p * INV_FACT[i];
            if i >= 5 || t.to_float().abs() <= inv_k * Double::EPSILON {
                break;
            }
        }

        s += t;

        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s += 1.0;

        s.ldexp(m as i32)
    }
}

// #endregion

// #region Logarithms

impl Double {
    pub fn ln(self) -> Double {
        // Strategy:
        //
        // The Taylor series for logarithms converges much more slowly than that of exp because of
        // the lack of a factorial term in the denominator. Hence this routine instead tries to
        // determine the root of the function
        //
        //      f(x) = exp(x) - a
        //
        // using Newton's iteration. This iteration is given by
        //
        //      x' = x - f(x)/f'(x)
        //         = x - (1 - a * exp(-x))
        //         = x + a * exp(-x) - 1
        //
        // So now we're doing a little calculus too. Exciting!
        //
        // Testing has shown that it requires two iterations to get the required precision.
        if self.is_one() {
            return Double::ZERO;
        }
        if self.is_zero() || self.is_sign_negative() {
            return Double::NAN;
        }

        let x1 = self.0.ln(); // initial approximation
        let x2 = x1 + self * (-x1).exp() - 1.0; // iteration 1
        x2 + self * (-x2).exp() - 1.0 // iteration 2
    }

    #[inline]
    pub fn log10(self) -> Double {
        self.ln() / Double::LN_10
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Double, b: Double) -> bool {
        (a - b).abs() < Double::from(10).powi(-30)
    }

    fn error_message(expected: Double, actual: Double) -> String {
        format!("\nExpected: {}\nActual:   {}", expected, actual)
    }

    #[test]
    fn exp() {
        let expected_e2: Double = "7.38905609893065022723042746057501".parse().unwrap();
        let expected_e3: Double = "20.0855369231876677409285296545817".parse().unwrap();
        let expected_e1_2: Double = "1.64872127070012814684865078781416".parse().unwrap();

        let actual_e2 = Double::from(2).exp();
        let actual_e3 = Double::from(3).exp();
        let actual_e1_2 = Double::from(0.5).exp();

        assert!(
            close(expected_e2, actual_e2),
            error_message(expected_e2, actual_e2)
        );
        assert!(
            close(expected_e3, actual_e3),
            error_message(expected_e3, actual_e3)
        );
        assert!(
            close(expected_e1_2, actual_e1_2),
            error_message(expected_e1_2, actual_e1_2)
        );
    }

    #[test]
    fn ln() {
        let expected_ln2: Double = "0.693147180559945309417232121458176".parse().unwrap();
        let expected_ln_ln2: Double = "-0.366512920581664327012439158232669".parse().unwrap();
        let expected_ln_pi: Double = "1.144729885849400174143427351353058".parse().unwrap();

        let actual_ln2 = Double::from(2).ln();
        let actual_ln_ln2 = actual_ln2.ln();
        let actual_ln_pi = Double::PI.ln();

        assert!(
            close(expected_ln2, actual_ln2),
            error_message(expected_ln2, actual_ln2)
        );
        assert!(
            close(expected_ln_ln2, actual_ln_ln2),
            error_message(expected_ln_ln2, actual_ln_ln2)
        );
        assert!(
            close(expected_ln_pi, actual_ln_pi),
            error_message(expected_ln_pi, actual_ln_pi)
        );
    }

    #[test]
    fn log10() {
        let expected_log_2: Double = "0.301029995663981195213738894724493".parse().unwrap();
        let expected_log_e: Double = "0.434294481903251827651128918916605".parse().unwrap();

        let actual_log_2 = Double::from(2).log10();
        let actual_log_e = Double::E.log10();

        assert!(
            close(expected_log_2, actual_log_2),
            error_message(expected_log_2, actual_log_2)
        );
        assert!(
            close(expected_log_e, actual_log_e),
            error_message(expected_log_e, actual_log_e)
        );
    }
}
