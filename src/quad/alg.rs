// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::*;
use crate::quad::common::mul_pwr2;
use crate::quad::Quad;

// #region Powers

impl Quad {
    /// Calculates the square of the number.
    ///
    /// This method takes advantage of optimizations in multiplication that are available when the
    /// two numbers being multiplied are the same, so it is more efficient than bare multiplication.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.sqr() == x * x); // The left side is faster though
    /// # }
    /// ```
    #[inline]
    pub fn sqr(self) -> Quad {
        // A considerable simplification over simply multiplying the number by itself, with the
        // simplifications possible because the two numbers being multiplied are in fact equal.
        //
        // The result is a simpler calculation:
        //
        //      a0² + 2a0a1 + 2a0a2 + a1² + 2a0a3 + 2a1a2
        //
        // where any further terms, including the low words of the final two terms, are unnecessary
        // to achieve the desired accuracy.

        let (h0, l0) = two_sqr(self.0);
        let (h1, l1) = two_prod(2.0 * self.0, self.1);
        let (h2, l2) = two_prod(2.0 * self.0, self.2);
        let (h3, l3) = two_sqr(self.1);
        let h4 = 2.0 * self.0 * self.3;
        let h5 = 2.0 * self.1 * self.2;

        // Less primitive functions are not used here because there are steps in them that can be
        // skipped.

        let r0 = h0;

        let (r1, a1) = two_sum(h1, l0);

        let (b0, b1) = two_sum(a1, l1);
        let (c0, c1) = two_sum(h2, h3);
        let (d0, d1) = two_sum(b0, c0);
        let (e0, e1) = two_sum(b1, c1);
        let (f0, f1) = two_sum(d1, e0);
        let (i0, i1) = quick_two_sum(f0, e1 + f1);
        let (r2, j1) = quick_two_sum(d0, i0);

        let (k0, k1) = quick_two_sum(i1, j1);
        let (m0, m1) = two_sum(h4, h5);
        let (n0, n1) = two_sum(l2, l3);
        let (o0, o1) = two_sum(m0, n0);
        let (r3, q1) = two_sum(k0, o0);

        let r4 = m1 + n1 + o1 + k1 + q1;

        Quad::from(renorm5(r0, r1, r2, r3, r4))
    }

    /// Calculates the number raised to an integral power.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.powi(3) == qd!(27));
    /// # }
    /// ```
    pub fn powi(self, n: i32) -> Quad {
        if n == 0 {
            if self.is_zero() {
                Quad::NAN
            } else {
                Quad::ONE
            }
        } else {
            let mut r = self;
            let mut s = Quad::ONE;
            let mut k = n.abs();

            if k > 1 {
                while k > 0 {
                    if k % 2 == 1 {
                        s *= r;
                    }
                    k /= 2;
                    if k > 0 {
                        r = r.sqr();
                    }
                }
            } else {
                s = r;
            }

            if n < 0 {
                s.recip()
            } else {
                s
            }
        }
    }

    /// Calculates the number raised to a quad-double power.
    ///
    /// This function only works for positive values of the number, as it uses a simplified
    /// logarithm-based algorithm. Full algorithms are much more difficult (see [this libm
    /// implementation][1] if you're curious) and it will take some time before there is such an
    /// implementation here.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3).powf(qd!(3.3));
    /// let expected = qd!("37.54050759852955219310186595463382927684873090166843452920390518");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn powf(self, n: Quad) -> Quad {
        // a^b = exp(b ln(a)), but since ln(a) is not defined for negative values, this works
        // ONLY FOR POSITIVE VALUES OF A (self in this case). Other solutions to powf are more
        // general but also much more complex and I am not yet ready to try one.
        (n * self.ln()).exp()
    }

    /// Calculates the number times 2<sup>`n`</sup>.
    ///
    /// Though this is not an everyday operation, it is often used in more advanced mathematical
    /// calculations (including several within this library). Therefore an implementation that is
    /// much more efficient than calculating it through multiplication and [`powi`](#method.powi) is
    /// offered despite it not being part of the `f64` API.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.ldexp(3) == qd!(24)); // 3 * 2^3
    /// # }
    /// ```
    #[inline]
    pub fn ldexp(self, n: i32) -> Quad {
        let factor = 2f64.powi(n);
        Quad(
            self.0 * factor,
            self.1 * factor,
            self.2 * factor,
            self.3 * factor,
        )
    }
}

// #endregion

// #region Roots

impl Quad {
    /// Calculates the square root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).sqrt();
    /// let expected = qd!("1.414213562373095048801688724209698078569671875376948073176679738");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn sqrt(self) -> Quad {
        if self.is_zero() {
            Quad::ZERO
        } else if self.is_sign_negative() {
            Quad::NAN
        } else {
            // Strategy: use a Newton iteration.
            //
            // Perform the following Newton iteration
            //
            //      x' = x + (1 - ax²) * x / 2
            //
            // which converges to 1/√a, starting with a double-precision approximation of 1/√a.
            // Newton's iteration more or less doubles the precision with each pass, so performing
            // it three times should be enough.

            let mut r = Quad::ONE / Quad::from(self.0.sqrt());
            let h = mul_pwr2(self, 0.5);
            let k = Quad(0.5, 0.0, 0.0, 0.0);

            r += (k - h * r.sqr()) * r;
            r += (k - h * r.sqr()) * r;
            r += (k - h * r.sqr()) * r;

            r *= self;
            r
        }
    }

    /// Calculates the cube root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).cbrt();
    /// let expected = qd!("1.259921049894873164767210607278228350570251464701507980081975112");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn cbrt(self) -> Quad {
        self.nroot(3)
    }

    /// Calculates the `n`th root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).nroot(4);
    /// let expected = qd!("1.189207115002721066717499970560475915292972092463817413019002225");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn nroot(self, n: i32) -> Quad {
        if n <= 0 {
            return Quad::NAN;
        }
        if n % 2 == 0 && self.is_sign_negative() {
            return Quad::NAN;
        }
        if n == 1 {
            return self;
        }
        if n == 2 {
            return self.sqrt(); // use the more specialized method in sqrt
        }
        if self.is_zero() {
            return Quad::ZERO;
        }

        // Strategy: the traditional way of finding roots is using Newton's iteration for the
        // function
        //
        //      f(x) = x^(-n) - a
        //
        // to find its root a^(-1/n). The iteration is therefore
        //
        //      x' = x + x * (1 - a * x^n) / n
        //
        // This converges quadratically, which is pretty fast. After performing a small number of
        // iterations, we can then find a^(1/n) by taking the reciprocal.

        let r = self.abs();
        let mut x: Quad = (-(r.0.ln()) / n as f64).exp().into(); // a^(-1/n) = exp(-ln(a) / n)

        let qd_n = Quad::from(n);
        x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
        x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
        x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
        if self.0 < 0.0 {
            x = -x
        }
        x.recip()
    }
}

// #endregion
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_alg_sqr() {
        assert_exact!(Quad::NAN, Quad::NAN.sqr());
        assert_exact!(Quad::ZERO, qd!(0).sqr());
        assert_exact!(qd!(121), qd!(-11).sqr());
        assert_close!(
            qd!("9.869604401089358618834490999876151135313699407240790626413349376"),
            Quad::PI.sqr()
        );
    }

    #[test]
    fn quad_alg_powi() {
        assert_exact!(Quad::NAN, Quad::NAN.powi(3));
        assert_exact!(Quad::ZERO, qd!(0).powi(3));
        assert_exact!(Quad::NAN, qd!(0).powi(0));
        assert_close!(
            qd!("-6.209213230591551744478457134696462611222531992971170622970363425e-6"),
            qd!(-11).powi(-5)
        );
        assert_close!(
            qd!("97.40909103400243723644033268870511124972758567268542169146785939"),
            Quad::PI.powi(4)
        );
    }

    #[test]
    fn quad_alg_powf() {
        assert_exact!(Quad::NAN, Quad::NAN.powf(qd!(3.6)));
        assert_exact!(Quad::NAN, qd!(0).powf(qd!(3.2))); // Sigh
        assert_exact!(Quad::NAN, qd!(0).powf(qd!(0)));
        assert_exact!(Quad::NAN, qd!(-1).powf(qd!(1))); // Also sigh
        assert_close!(
            qd!("24567.24805421478199532529771567617705237167216222778116359595012"),
            qd!(11.1).powf(qd!(4.2))
        );
        assert_close!(
            qd!("1.409759279075053716836003243441716711042960485535248677014414790"),
            Quad::PI.powf(qd!(0.3))
        );
    }

    #[test]
    fn quad_alg_ldexp() {}

    #[test]
    fn quad_alg_sqrt() {}

    #[test]
    fn quad_alg_cbrt() {}

    #[test]
    fn quad_alg_nroot() {}
}
