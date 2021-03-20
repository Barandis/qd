// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

pub(super) const SINES: [Double; 4] = [
    Double(1.950_903_220_161_282_8e-1, -7.991_079_068_461_734e-18),
    Double(3.826_834_323_650_898e-1, -1.005_077_269_646_159e-17),
    Double(5.555_702_330_196_022e-1, 4.709_410_940_561_675_6e-17),
    Double(7.071_067_811_865_476e-1, -4.833_646_656_726_457_3e-17),
];

pub(super) const COSINES: [Double; 4] = [
    Double(9.807_852_804_032_304e-1, 1.854_693_999_782_499_6e-17),
    Double(9.238_795_325_112_867e-1, 1.764_504_708_433_667e-17),
    Double(8.314_696_123_025_452e-1, 1.407_385_698_472_800_8e-18),
    Double(7.071_067_811_865_476e-1, -4.833_646_656_726_457_3e-17),
];
