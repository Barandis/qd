var searchIndex = JSON.parse('{\
"qd":{"doc":"Rust implementation of double-double and quad-double …","i":[[0,"error","qd","Errors that may occur while parsing a string into a <code>Double</code>…",null,null],[3,"ParseDoubleError","qd::error","An error generated when a problem is encountered parsing …",null,null],[12,"kind","","",0,null],[3,"ParseQuadError","","An error generated when a problem is encountered parsing …",null,null],[12,"kind","","",1,null],[4,"ErrorKind","","The different kinds of errors that might be generated …",null,null],[13,"Empty","","An error indicating that an attempt was made to parse an …",2,null],[13,"Invalid","","An error indicating that the format of a parsed string is …",2,null],[3,"Double","qd","A 128-bit floating-point number implemented as the …",null,null],[3,"Quad","","A 256-bit floating-point number implemented as the …",null,null],[14,"dd","","Creates a new double-double from another number or from a …",null,null],[14,"qd","","Creates a new quad-double from another number or from a …",null,null],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"to_string","","",3,[[],["string",3]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"to_string","","",4,[[],["string",3]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","qd::error","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"to_string","","",0,[[],["string",3]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","qd","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","",3,[[],["double",3]]],[11,"from","","Parses a string to create a <code>Double</code>.",3,[[],["double",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","",4,[[],["quad",3]]],[11,"from","","Generates a <code>Quad</code> from a <code>Double</code>.",4,[[["double",3]],["quad",3]]],[11,"from","","Parses a string to create a <code>Quad</code>.",4,[[],["quad",3]]],[11,"clone","","",3,[[],["double",3]]],[11,"clone","","",4,[[],["quad",3]]],[11,"clone","qd::error","",0,[[],["parsedoubleerror",3]]],[11,"clone","","",1,[[],["parsequaderror",3]]],[11,"clone","","",2,[[],["errorkind",4]]],[11,"default","qd","",3,[[],["double",3]]],[11,"eq","","Implements the <code>==</code> and <code>!= operators, testing two </code>Double`s …",3,[[["double",3]]]],[11,"eq","","Implements the <code>==</code> and <code>!= operators, testing two </code>Quad`s …",4,[[["quad",3]]]],[11,"eq","qd::error","",0,[[["parsedoubleerror",3]]]],[11,"ne","","",0,[[["parsedoubleerror",3]]]],[11,"eq","","",1,[[["parsequaderror",3]]]],[11,"ne","","",1,[[["parsequaderror",3]]]],[11,"eq","","",2,[[["errorkind",4]]]],[11,"partial_cmp","qd","Implements the <code><</code>, <code>></code>, <code><=</code>, and <code>>=</code> operators, testing two …",3,[[["double",3]],[["option",4],["ordering",4]]]],[11,"partial_cmp","","Implements the <code><</code>, <code>></code>, <code><=</code>, and <code>>=</code> operators, testing two …",4,[[["quad",3]],[["option",4],["ordering",4]]]],[11,"fmt","","Formats a <code>Double</code> for display when the \\\"<code>?</code>\\\" formatting …",3,[[["formatter",3]],["result",6]]],[11,"fmt","","Formats a <code>Double</code> for display when the \\\"<code>?</code>\\\" formatting …",4,[[["formatter",3]],["result",6]]],[11,"fmt","qd::error","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"fmt","qd","Formats a <code>Double</code> for display.",3,[[["formatter",3]],["result",6]]],[11,"fmt","","Formats a <code>Quad</code> for display.",4,[[["formatter",3]],["result",6]]],[11,"fmt","qd::error","Displays an English-language message describing the kind …",0,[[["formatter",3]],["result",6]]],[11,"fmt","","Displays an English-language message describing the kind …",1,[[["formatter",3]],["result",6]]],[11,"fmt","qd","Formats a <code>Double</code> for display when the \\\"<code>e</code>\\\" formatting …",3,[[["formatter",3]],["result",6]]],[11,"fmt","","Formats a <code>Quad</code> for display when the \\\"<code>e</code>\\\" formatting option …",4,[[["formatter",3]],["result",6]]],[11,"fmt","","Formats a <code>Double</code> for display when the \\\"<code>E</code>\\\" formatting …",3,[[["formatter",3]],["result",6]]],[11,"fmt","","Formats a <code>Double</code> for display when the \\\"<code>E</code>\\\" formatting …",4,[[["formatter",3]],["result",6]]],[11,"div","","Divides this <code>Double</code> by another, producing a new <code>Double</code> as …",3,[[["double",3]],["double",3]]],[11,"div","","Divides this <code>Double</code> by a reference to another <code>Double</code>, …",3,[[["double",3]],["double",3]]],[11,"div","","Divides this <code>Quad</code> by another, producing a new <code>Quad</code> as a …",4,[[["quad",3]],["quad",3]]],[11,"div","","Divides this <code>Quad</code> by a reference to another, producing a …",4,[[["quad",3]],["quad",3]]],[11,"rem","","Divides this <code>Double</code> by another, producing a new <code>Double</code> of …",3,[[["double",3]],["double",3]]],[11,"rem","","Divides this <code>Double</code> by a reference to another, producing …",3,[[["double",3]],["double",3]]],[11,"rem","","Divides this <code>Quad</code> by another, producing a new <code>Quad</code> of the …",4,[[["quad",3]],["quad",3]]],[11,"rem","","Divides this <code>Quad</code> by a reference to another, producing a …",4,[[["quad",3]],["quad",3]]],[11,"sub","","Subtracts another <code>Double</code> from this one, producing a new …",3,[[["double",3]],["double",3]]],[11,"sub","","Subtracts another reference to a <code>Double</code> from this <code>Double</code>, …",3,[[["double",3]],["double",3]]],[11,"sub","","Subtracts another <code>Quad</code> from this one, producing a new <code>Quad</code>…",4,[[["quad",3]],["quad",3]]],[11,"sub","","Subtracts a reference to another <code>Quad</code> from this <code>Quad</code>, …",4,[[["quad",3]],["quad",3]]],[11,"add","","Adds this <code>Double</code> to another, producing a new <code>Double</code> as a …",3,[[["double",3]],["double",3]]],[11,"add","","Adds this <code>Double</code> to a reference to another <code>Double</code>, …",3,[[["double",3]],["double",3]]],[11,"add","","Adds this <code>Quad</code> to another, producing a new <code>Quad</code> as a …",4,[[["quad",3]],["quad",3]]],[11,"add","","Adds this <code>Quad</code> to a reference to another, producing a new …",4,[[["quad",3]],["quad",3]]],[11,"mul","","Multiplies this <code>Double</code> by another, producing a new <code>Double</code> …",3,[[["double",3]],["double",3]]],[11,"mul","","Multiplies this <code>Double</code> by a reference to another, …",3,[[["double",3]],["double",3]]],[11,"mul","","Multiplies this <code>Quad</code> by another, producing a new <code>Quad</code> as …",4,[[["quad",3]],["quad",3]]],[11,"mul","","Multiplies this <code>Quad</code> by a reference to another, producing …",4,[[["quad",3]],["quad",3]]],[11,"neg","","Negates this <code>Double</code>, producing a new <code>Double</code>.",3,[[],["double",3]]],[11,"neg","","Negates this <code>Quad</code>, producing a new <code>Quad</code>.",4,[[],["quad",3]]],[11,"add_assign","","Adds another <code>Double</code> to this one, modifying this one to …",3,[[["double",3]]]],[11,"add_assign","","Adds a reference to another <code>Double</code> to this <code>Double</code>, …",3,[[["double",3]]]],[11,"add_assign","","Adds another <code>Quad</code> to this one, modifying this one to …",4,[[["quad",3]]]],[11,"add_assign","","Adds a reference to another <code>Quad</code> to this one, modifying …",4,[[["quad",3]]]],[11,"sub_assign","","Subtracts another <code>Double</code> from this one, modifying this …",3,[[["double",3]]]],[11,"sub_assign","","Subtracts a reference to another <code>Double</code> from this one, …",3,[[["double",3]]]],[11,"sub_assign","","Subtracts another <code>Quad</code> from this one, modifying this one …",4,[[["quad",3]]]],[11,"sub_assign","","Subtracts a reference to another <code>Quad</code> from this <code>Quad</code>, …",4,[[["quad",3]]]],[11,"mul_assign","","Multiples this <code>Double</code> by another one, modifying this one …",3,[[["double",3]]]],[11,"mul_assign","","Multiples this <code>Double</code> by a reference to another one, …",3,[[["double",3]]]],[11,"mul_assign","","Multiples this <code>Quad</code> by another one, modifying this one to …",4,[[["quad",3]]]],[11,"mul_assign","","Multiples this <code>Quad</code> by a reference to another one, …",4,[[["quad",3]]]],[11,"div_assign","","Divides this <code>Double</code> by another, modifying this one to …",3,[[["double",3]]]],[11,"div_assign","","Divides this <code>Double</code> by a reference to another, modifying …",3,[[["double",3]]]],[11,"div_assign","","Divides this <code>Quad</code> by another, modifying this one to equal …",4,[[["quad",3]]]],[11,"div_assign","","Divides this <code>Quad</code> by a reference to another, modifying …",4,[[["quad",3]]]],[11,"rem_assign","","Divides this <code>Double</code> by another, modifying this one to …",3,[[["double",3]]]],[11,"rem_assign","","Divides this <code>Double</code> by a reference to another, modifying …",3,[[["double",3]]]],[11,"rem_assign","","Divides this <code>Quad</code> by another, modifying this one to equal …",4,[[["quad",3]]]],[11,"rem_assign","","Divides this <code>Quad</code> by a reference to another, modifying …",4,[[["quad",3]]]],[11,"index","","Returns one of the components of the <code>Double</code>.",3,[[]]],[11,"index","","Returns one of the components of the <code>Quad</code>.",4,[[]]],[11,"from_str","","Parses a string to create a <code>Double</code>.",3,[[],[["double",3],["result",4],["parsedoubleerror",3]]]],[11,"from_str","","Parses a string to create a <code>Quad</code>.",4,[[],[["result",4],["parsequaderror",3],["quad",3]]]],[11,"sum","","Sums all of the values in an iterator of <code>Double</code>s.",3,[[],["double",3]]],[11,"sum","","Sums all of the referenced values in an iterator of <code>Double</code>…",3,[[],["double",3]]],[11,"sum","","Sums all of the values in an iterator of <code>Quad</code>s.",4,[[],["quad",3]]],[11,"sum","","Sums all of the referenced values in an iterator of <code>Quad</code>s.",4,[[],["quad",3]]],[11,"product","","Multiplies all of the values in an iterator of <code>Double</code>s.",3,[[],["double",3]]],[11,"product","","Multiples all of the referenced values in an iterator of …",3,[[],["double",3]]],[11,"product","","Multiplies all of the values in an iterator of <code>Quad</code>s.",4,[[],["quad",3]]],[11,"product","","Multiples all of the referenced values in an iterator of …",4,[[],["quad",3]]],[18,"RADIX","","The radix or base of the internal representation of <code>Double</code>…",3,null],[18,"MANTISSA_DIGITS","","Number of significant digits in base 2.",3,null],[18,"DIGITS","","Approximate number of significant digits in base 10.",3,null],[18,"EPSILON","","Machine epsilon value for <code>Double</code>.",3,null],[18,"MIN","","Smallest finite <code>Double</code> value.",3,null],[18,"MIN_POSITIVE","","Smallest positive normal <code>Double</code> value.",3,null],[18,"MAX","","Largest finite <code>Double</code> value.",3,null],[18,"MIN_EXP","","One greater than the minimum possible normal power of 2 …",3,null],[18,"MAX_EXP","","Maximum possible power of 2 exponent.",3,null],[18,"MIN_10_EXP","","Minimum possible normal power of 10 exponent.",3,null],[18,"MAX_10_EXP","","Maximum possible power of 10 exponent.",3,null],[18,"NAN","","Not a Number (NaN).",3,null],[18,"INFINITY","","Infinity (∞).",3,null],[18,"NEG_INFINITY","","Negative infinity (-∞).",3,null],[18,"ZERO","","Zero (0)",3,null],[18,"NEG_ZERO","","Negative zero (-0)",3,null],[18,"ONE","","One (1)",3,null],[18,"NEG_ONE","","Negative one (-1)",3,null],[18,"PI","","Archimedes\' constant (π)",3,null],[18,"TAU","","The full circle constant (τ), or 2π",3,null],[18,"FRAC_PI_2","","π/2",3,null],[18,"FRAC_PI_3","","π/3",3,null],[18,"FRAC_PI_4","","π/4",3,null],[18,"FRAC_PI_6","","π/6",3,null],[18,"FRAC_PI_8","","π/8",3,null],[18,"FRAC_PI_16","","π/16",3,null],[18,"FRAC_3_PI_2","","3π/2",3,null],[18,"FRAC_3_PI_4","","3π/4",3,null],[18,"FRAC_5_PI_4","","5π/4",3,null],[18,"FRAC_7_PI_4","","7π/4",3,null],[18,"FRAC_1_PI","","1/π",3,null],[18,"FRAC_2_PI","","2/π",3,null],[18,"FRAC_2_SQRT_PI","","2/√π",3,null],[18,"SQRT_2","","√2",3,null],[18,"FRAC_1_SQRT_2","","1/√2",3,null],[18,"E","","Euler\'s number (e)",3,null],[18,"LOG2_10","","log₂ 10",3,null],[18,"LOG2_E","","log₂ e",3,null],[18,"LOG10_2","","log 2",3,null],[18,"LOG10_E","","log e",3,null],[18,"LN_2","","ln 2",3,null],[18,"LN_10","","ln 10",3,null],[11,"ldexp","","Calculates x · 2n, where <em>x</em> is the <code>Double</code> and <em>n</em> is an …",3,[[],["double",3]]],[11,"sqr","","Calculates the square of the <code>Double</code>.",3,[[],["double",3]]],[11,"sqrt","","Calculates the square root of the <code>Double</code>.",3,[[],["double",3]]],[11,"nroot","","Calculates the <em>n</em>th root of the <code>Double</code>.",3,[[],["double",3]]],[11,"cbrt","","Calculates the cube root of the <code>Double</code>.",3,[[],["double",3]]],[11,"powi","","Calculates the <code>Double</code> raised to an integer power.",3,[[],["double",3]]],[11,"powf","","Calculates the <code>Double</code> raised to a <code>Quad</code> power.",3,[[["double",3]],["double",3]]],[11,"recip","","Calculates the reciprocal of the <code>Double</code>.",3,[[],["double",3]]],[11,"sinh_cosh","","Simultaneously computes the hyperbolic sine and cosine …",3,[[]]],[11,"sinh","","Computes the hyperbolic sine (sinh) of the <code>Double</code>.",3,[[],["double",3]]],[11,"cosh","","Computes the hyperbolic cosine (cosh) of the <code>Double</code>.",3,[[],["double",3]]],[11,"tanh","","Computes the hyperbolic tangent (tanh) of the <code>Double</code>.",3,[[],["double",3]]],[11,"asinh","","Calculates the inverse hyperbolic sine (sinh-1) of the …",3,[[],["double",3]]],[11,"acosh","","Calculates the inverse hyperbolic cosine (cosh-1) of the …",3,[[],["double",3]]],[11,"atanh","","Calculates the inverse hyperbolic tangent (tanh-1) of the …",3,[[],["double",3]]],[11,"abs","","Calculates the absolute value of the <code>Double</code>.",3,[[],["double",3]]],[11,"floor","","Returns the largest integer value less than or equal to …",3,[[],["double",3]]],[11,"ceil","","Returns the smallest integer value greater than or equal …",3,[[],["double",3]]],[11,"round","","Returns the nearest integer value to the <code>Double</code>. Half-way …",3,[[],["double",3]]],[11,"trunc","","Returns the integer part of the <code>Double</code>.",3,[[],["double",3]]],[11,"fract","","Returns the fractional part of the <code>Double</code>.",3,[[],["double",3]]],[11,"signum","","Returns a number that represents the sign of the <code>Double</code>.",3,[[],["double",3]]],[11,"classify","","Returns the floating point category of the <code>Double</code>.",3,[[],["fpcategory",4]]],[11,"is_normal","","Returns <code>true</code> if the <code>Double</code> is neither zero, infinite, …",3,[[]]],[11,"is_zero","","Returns <code>true</code> if the <code>Double</code> is either positive or negative …",3,[[]]],[11,"is_sign_negative","","Returns <code>true</code> if the <code>Double</code> is negative, including …",3,[[]]],[11,"is_sign_positive","","Returns <code>true</code> if the <code>Double</code> is positive, including …",3,[[]]],[11,"is_nan","","Returns <code>true</code> if the <code>Double</code> is <code>NaN</code>.",3,[[]]],[11,"is_infinite","","Returns <code>true</code> if the <code>Double</code> is positive or negative …",3,[[]]],[11,"is_finite","","Returns <code>true</code> if the <code>Double</code> is neither infinite nor <code>NaN</code>.",3,[[]]],[11,"is_subnormal","","Returns <code>true</code> if the <code>Double</code> has an absolute value of less …",3,[[]]],[11,"exp","","Computes the exponential function, <em>e</em>x, where <em>x</em> is this …",3,[[],["double",3]]],[11,"ln","","Calculates the natural logarithm, log<em>e</em>, of the <code>Double</code>.",3,[[],["double",3]]],[11,"log10","","Calculates the base-10 logarithm, log10, of the <code>Double</code>.",3,[[],["double",3]]],[11,"log2","","Calculates the base-2 logarithm, log2, of the <code>Double</code>.",3,[[],["double",3]]],[11,"log","","Calculates the base <code>b</code> logarithm (log<code>b</code>) of the <code>Double</code>.",3,[[["double",3]],["double",3]]],[11,"sin_cos","","Simultaneously computes the sine (sin) and the cosine …",3,[[]]],[11,"sin","","Computes the sine (sin) of the <code>Double</code>.",3,[[],["double",3]]],[11,"cos","","Computes the cosine (cos) of the <code>Double</code>.",3,[[],["double",3]]],[11,"tan","","Computes the tangent (tan) of the <code>Double</code>.",3,[[],["double",3]]],[11,"atan2","","Computes the 2-argument inverse tangent (tan-1) of this …",3,[[["double",3]],["double",3]]],[11,"asin","","Computes the inverse sine (sin-1) of the <code>Double</code>. The …",3,[[],["double",3]]],[11,"acos","","Computes the inverse cosine (cos-1) of the <code>Double</code>. The …",3,[[],["double",3]]],[11,"atan","","Computes the inverse tangent (tan-1) of the <code>Double</code>. The …",3,[[],["double",3]]],[11,"new","","Creates a <code>Double</code> with the two arguments as the internal …",3,[[],["double",3]]],[18,"RADIX","","The radix or base of the internal representation of <code>Double</code>…",4,null],[18,"MANTISSA_DIGITS","","Number of significant digits in base 2.",4,null],[18,"DIGITS","","Approximate number of significant digits in base 10.",4,null],[18,"EPSILON","","Machine epsilon value for <code>Quad</code>.",4,null],[18,"MIN","","Smallest finite <code>Quad</code> value.",4,null],[18,"MIN_POSITIVE","","Smallest positive normal <code>Quad</code> value.",4,null],[18,"MAX","","Largest finite <code>Quad</code> value.",4,null],[18,"MIN_EXP","","One greater than the minimum possible normal power of 2 …",4,null],[18,"MAX_EXP","","Maximum possible power of 2 exponent.",4,null],[18,"MIN_10_EXP","","Minimum possible normal power of 10 exponent.",4,null],[18,"MAX_10_EXP","","Maximum possible power of 10 exponent.",4,null],[18,"NAN","","Not a Number (NaN).",4,null],[18,"INFINITY","","Infinity (∞).",4,null],[18,"NEG_INFINITY","","Negative infinity (-∞).",4,null],[18,"ZERO","","Zero (0)",4,null],[18,"NEG_ZERO","","Negative zero (-0)",4,null],[18,"ONE","","One (1)",4,null],[18,"NEG_ONE","","Negative one (-1)",4,null],[18,"PI","","Archimedes\' constant (π)",4,null],[18,"TAU","","The full circle constant (τ), or 2π",4,null],[18,"FRAC_PI_2","","π/2",4,null],[18,"FRAC_PI_3","","π/3",4,null],[18,"FRAC_PI_4","","π/4",4,null],[18,"FRAC_PI_6","","π/6",4,null],[18,"FRAC_PI_8","","π/8",4,null],[18,"FRAC_PI_16","","π/16",4,null],[18,"FRAC_3_PI_2","","3π/2",4,null],[18,"FRAC_3_PI_4","","3π/4",4,null],[18,"FRAC_5_PI_4","","5π/4",4,null],[18,"FRAC_7_PI_4","","7π/4",4,null],[18,"FRAC_1_PI","","1/π",4,null],[18,"FRAC_2_PI","","2/π",4,null],[18,"FRAC_2_SQRT_PI","","2/√π",4,null],[18,"SQRT_2","","√2",4,null],[18,"FRAC_1_SQRT_2","","1/√2",4,null],[18,"E","","Euler\'s number (e)",4,null],[18,"LOG2_10","","log₂ 10",4,null],[18,"LOG2_E","","log₂ e",4,null],[18,"LOG10_2","","log 2",4,null],[18,"LOG10_E","","log e",4,null],[18,"LN_2","","ln 2",4,null],[18,"LN_10","","ln 10",4,null],[11,"ldexp","","Calculates x · 2n, where <em>x</em> is the <code>Quad</code> and <em>n</em> is an …",4,[[],["quad",3]]],[11,"sqr","","Calculates the square of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"sqrt","","Calculates the square root of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"nroot","","Calculates the <em>n</em>th root of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"cbrt","","Calculates the cube root of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"powi","","Calculates the <code>Quad</code> raised to an integer power.",4,[[],["quad",3]]],[11,"powf","","Calculates the <code>Quad</code> raised to a <code>Quad</code> power.",4,[[["quad",3]],["quad",3]]],[11,"recip","","Calculates the reciprocal of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"sinh_cosh","","Simultaneously computes the hyperbolic sine and cosine …",4,[[]]],[11,"sinh","","Computes the hyperbolic sine (sinh) of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"cosh","","Computes the hyperbolic cosine (cosh) of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"tanh","","Computes the hyperbolic tangent (tanh) of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"asinh","","Calculates the inverse hyperbolic sine (sinh-1) of the …",4,[[],["quad",3]]],[11,"acosh","","Calculates the inverse hyperbolic cosine (cosh-1) of the …",4,[[],["quad",3]]],[11,"atanh","","Calculates the inverse hyperbolic tangent (tanh-1) of the …",4,[[],["quad",3]]],[11,"abs","","Calculates the absolute value of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"floor","","Returns the largest integer value less than or equal to …",4,[[],["quad",3]]],[11,"ceil","","Returns the smallest integer value greater than or equal …",4,[[],["quad",3]]],[11,"round","","Returns the nearest integer value to the <code>Double</code>. Half-way …",4,[[],["quad",3]]],[11,"trunc","","Returns the integer part of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"fract","","Returns the fractional part of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"signum","","Returns a number that represents the sign of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"classify","","Returns the floating point category of the <code>Quad</code>.",4,[[],["fpcategory",4]]],[11,"is_normal","","Returns <code>true</code> if the <code>Quad</code> is neither zero, infinite, …",4,[[]]],[11,"is_zero","","Returns <code>true</code> if the <code>Quad</code> is either positive or negative …",4,[[]]],[11,"is_sign_negative","","Returns <code>true</code> if the <code>Quad</code> is negative, including negative …",4,[[]]],[11,"is_sign_positive","","Returns <code>true</code> if the <code>Quad</code> is positive, including positive …",4,[[]]],[11,"is_nan","","Returns <code>true</code> if the <code>Quad</code> is <code>NaN</code>.",4,[[]]],[11,"is_infinite","","Returns <code>true</code> if the <code>Quad</code> is positive or negative infinity.",4,[[]]],[11,"is_finite","","Returns <code>true</code> if the <code>Quad</code> is neither infinite nor <code>NaN</code>..",4,[[]]],[11,"is_subnormal","","Returns <code>true</code> if the <code>Quad</code> has an absolute value of less …",4,[[]]],[11,"exp","","Computes the exponential function, <em>e</em>x, where <em>x</em> is this …",4,[[],["quad",3]]],[11,"ln","","Calculates the natural logarithm, log<em>e</em>, of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"log10","","Calculates the base-10 logarithm, log10, of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"log2","","Calculates the base-2 logarithm, log2, of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"log","","Calculates the base <code>b</code> logarithm (log<code>b</code>) of the <code>Quad</code>.",4,[[["quad",3]],["quad",3]]],[11,"sin_cos","","Simultaneously computes the sine (sin) and the cosine …",4,[[]]],[11,"sin","","Computes the sine (sin) of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"cos","","Computes the cosine (cos) of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"tan","","Computes the tangent (tan) of the <code>Quad</code>.",4,[[],["quad",3]]],[11,"atan2","","Computes the 2-argument inverse tangent (tan-1) of this …",4,[[["quad",3]],["quad",3]]],[11,"asin","","Computes the inverse sine (sin-1) of the <code>Quad</code>. The domain …",4,[[],["quad",3]]],[11,"acos","","Computes the inverse cosine (cos-1) of the <code>Quad</code>. The …",4,[[],["quad",3]]],[11,"atan","","Computes the inverse tangent (tan-1) of the <code>Quad</code>. The …",4,[[],["quad",3]]],[11,"new","","Creates a <code>Quad</code> with the four arguments as the internal …",4,[[],["quad",3]]]],"p":[[3,"ParseDoubleError"],[3,"ParseQuadError"],[4,"ErrorKind"],[3,"Double"],[3,"Quad"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);