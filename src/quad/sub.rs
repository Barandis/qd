// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::ops::{Add, Sub, SubAssign};

impl Sub for Quad {
    type Output = Quad;

    /// Subtracts another `Quad` from this one, producing a new `Quad` as a result.
    ///
    /// This implements the binary `-` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E - Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn sub(self, other: Quad) -> Quad {
        self.add(-other)
    }
}

impl Sub for &Quad {
    type Output = Quad;

    /// Subtracts a reference to another `Quad` from this one, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the binary `-` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::E - &Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn sub(self, other: &Quad) -> Quad {
        (*self).add(-*other)
    }
}

impl Sub<&Quad> for Quad {
    type Output = Quad;

    /// Subtracts a reference to another `Quad` from this `Quad`, producing a new `Quad` as
    /// a result.
    ///
    /// This implements the binary `-` operator between a `Quad` and a reference to a
    /// `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E - &Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn sub(self, other: &Quad) -> Quad {
        self.add(-*other)
    }
}

impl Sub<Quad> for &Quad {
    type Output = Quad;

    /// Subtracts another `Quad` from a reference to this one, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the binary `-` operator between a reference to a `Quad` and a
    /// `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::E - Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn sub(self, other: Quad) -> Quad {
        (*self).add(-other)
    }
}

impl SubAssign for Quad {
    /// Subtracts another `Quad` from this one, modifying this one to equal the result.
    ///
    /// This implements the `-=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::E;
    /// x -= Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Quad) {
        let r = self.add(-other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl SubAssign<&Quad> for Quad {
    /// Subtracts a reference to another `Quad` from this `Quad`, modifying this one to
    /// equal the result.
    ///
    /// This implements the `-=` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::E;
    /// x -= &Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Quad) {
        let r = self.add(-*other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // sub tests
    test_all_near!(
        num_num:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI - Quad::E;
        num_ref:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI - &Quad::E;
        ref_num:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            &Quad::PI - Quad::E;
        ref_ref:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            &Quad::PI - &Quad::E;
        num_neg_num:
            qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"),
            Quad::PI - -Quad::E;
        num_neg_ref:
            qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"),
            Quad::PI - -&Quad::E;
        ref_neg_num:
            qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"),
            &Quad::PI - -Quad::E;
        ref_neg_ref:
            qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"),
            &Quad::PI - -&Quad::E;
        num_id:
            Quad::PI,
            Quad::PI - Quad::ZERO;
        id_num:
            -Quad::PI,
            Quad::ZERO - Quad::PI;
        num_small:
            qd!("3.1415926535897932384626433832795028841971693993751058209749435923073"),
            Quad::PI - qd!("1e-60");
        small_num:
            qd!("-3.1415926535897932384626433832795028841971693993751058209749435923073"),
            qd!("1e-60") - Quad::PI;
        three_nums:
            qd!("-0.26983635542919730631487620953133618163557782868510900811270304491058"),
            Quad::PI - Quad::E - Quad::LN_2;
        lassoc:
            qd!("-0.26983635542919730631487620953133618163557782868510900811270304491058"),
            (Quad::PI - Quad::E) - Quad::LN_2;
        rassoc:
            qd!("5.1667273014888931644056987331739888138789163587148101418212322105361"),
            Quad::PI - (Quad::LN_2 - Quad::E);
    );
    test_all_exact!(
        inf_one:
            Quad::INFINITY,
            Quad::INFINITY - Quad::ONE;
        one_inf:
            Quad::NEG_INFINITY,
            Quad::ONE - Quad::INFINITY;
        neg_inf_one:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY - Quad::ONE;
        one_neg_inf:
            Quad::INFINITY,
            Quad::ONE - Quad::NEG_INFINITY;
        inf_neg_inf:
            Quad::INFINITY,
            Quad::INFINITY - Quad::NEG_INFINITY;
        neg_inf_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY - Quad::INFINITY;

        nan_one:
            Quad::NAN,
            Quad::NAN - Quad::ONE;
        one_nan:
            Quad::NAN,
            Quad::ONE - Quad::NAN;
        nan_inf:
            Quad::NAN,
            Quad::NAN - Quad::INFINITY;
        inf_nan:
            Quad::NAN,
            Quad::INFINITY - Quad::NAN;
        inf_inf:
            Quad::NAN,
            Quad::INFINITY - Quad::INFINITY;
        neg_inf_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY - Quad::NEG_INFINITY;
    );

    // Assign tests. Assign code delegates to sub code, so there's no need to re-test all
    // of the cases above.
    test_all!(
        assign_num: {
            let mut a = Quad::PI;
            a -= Quad::E;
            near!(qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"), a);
        }
        assign_ref: {
            let mut b = Quad::PI;
            b -= &Quad::E;
            near!(qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"), b);
        }
    );
}
