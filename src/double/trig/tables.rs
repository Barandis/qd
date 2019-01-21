// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

pub(super) const SINES: [Double; 4] = [
    Double(1.9509032201612828e-1, -7.991079068461734e-18),
    Double(3.826834323650898e-1, -1.005077269646159e-17),
    Double(5.555702330196022e-1, 4.7094109405616756e-17),
    Double(7.071067811865476e-1, -4.8336466567264573e-17),
];

pub(super) const COSINES: [Double; 4] = [
    Double(9.807852804032304e-1, 1.8546939997824996e-17),
    Double(9.238795325112867e-1, 1.764504708433667e-17),
    Double(8.314696123025452e-1, 1.4073856984728008e-18),
    Double(7.071067811865476e-1, -4.8336466567264573e-17),
];
