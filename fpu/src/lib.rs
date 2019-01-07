#![feature(asm)]

#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
use std::mem::size_of;

// Setting precision is not necessary on non-x86 architectures, or when the x86 is using
// SSE/SSE2 extensions, because floating point numbers are calculated in the same precision
// as they're stored.
#[cfg(any(not(target_arch = "x86"), target_feature = "sse2"))]
pub fn set_precision<T>() {}

// On x86, the x87 FPU is used for float ops if the SSE/SSE2 extensions are not available.
// The FPU operates with 80 bits of precision by default, which means that operations will
// round to 80 bits and then round again when values are eventually represented as 32/64
// bit values. This causes problems with the precision of the last few bits in a floating
// point number that wreak havoc on high-precision calculations.
//
// We can overcome this by setting a control word in the FPU via assembly language so that
// computations are performed in the desired precision in the first place.

/// A structure used to preserve the original value of the FPU control word so that it
/// can be restored when the structure is dropped.
///
/// The x87 FPU control register is a 16-bit register with the following fields:
///
/// Bit   Field Description
/// ---------------------------------------------------------------------
/// EXCEPTION FLAG MASKS
/// 0     IM    Invalid Operation
/// 1     DM    Denormalized Operand
/// 2     ZM    Divide by Zero
/// 3     OM    Overflow
/// 4     UM    Undeflow
/// 5     PM    Precision
///
/// PRECISION CONTROL
/// 8,9   PC    Single Precision (32-bit)       0b00
///             Reserved                        0b01
///             Double Precision (64-bit)       0b10
///             Extended Precision (80-bit)     0b11
///
/// ROUNDING MODE
/// 10,11 RC    Round to nearest even           0b00
///             Round down towards infinity     0b01
///             Round up towards infinity       0b10
///             Round down towards 0 (truncate) 0b11
///
/// Bits 6, 7, 12, 13, 14, and 15 are reserved and not used.
///
/// The only field relevant for our purposes is PC, Precision Control. This field determines
/// the precision of the floating point operations performed by the FPU.
#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
pub struct FpuControlWord(u16);

/// Sets the FPU control word using the x86 instruction FLDCW - Load x87 FPU Control Word.
#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
fn set_cw(cw: u16) {
    unsafe {
        // The asm! macro generates GCC-like inline assembly code. This particular code
        // generates an instruction to load the contents of the memory location named `cw`
        // (i.e, the `cw` argument) into the FPU control register. The `volatile` makes sure
        // that the compiler doesn't optimize this call away.
        asm!("fldcw $0" : : "m" (cw) : : "volatile")
    }
}

/// Set the precision field of the FPU control register to the size of `T`. This function
/// returns the original FPU control word in a struct that allows the original to be
/// restored when the word goes out of scope.
///
/// For example, `let cw = set_precision::<f64>();` will set the CPU's floating point
/// precision to 64 bits (the size of an f64). When the return value goes out of scope, the
/// CPU's floating point precision will be set back to whatever it was before `set_precision`
/// was called.
///
/// The type `T` must be either 32, 64, or 80 bits in size (as these are the only values
/// available on the CPU). If a differently-sized `T` is passed, the precision will be set to
/// the default of 80 bits.
#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
pub fn set_precision<T>() -> FpuControlWord {
    let cw = 0u16;

    // Compute the bit value for the PC field appropriate for `T`.
    let cw_precision = match size_of::<T>() {
        4 => 0x0000, // 32 bits
        8 => 0x0200, // 64 bits
        _ => 0x0300, // 80 bits
    };

    unsafe {
        // The FNSTCW instruction - Store x87 FPU Control Word - returns the current value
        // of the FPU control word (in this case, bu storing it in the memory location
        // specified by the variable `cw`).
        asm!("fnstcw $0" : "=*m" (&cw) : : : "volatile")
    }

    // Set the control word to the desired precision. This works by masking away the PC
    // bits (8 and 9) and replacing them with the bits calculated above.
    set_cw((cw & 0xFCFF) | cw_precision);

    // Return the value that we got from FNSTCW so that value can be restored later.
    FpuControlWord(cw);
}

#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
impl Drop for FpuControlWord {
    /// Called when the value is about to be dropped (i.e., when no variable owns it anymore).
    /// This simply sets the FPU control word to the value stored within the struct itself.
    fn drop(&mut self) {
        set_cw(self.0)
    }
}
