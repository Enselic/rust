//@ run-pass
//@ compile-flags: -Z mir-opt-level=2

// Regression test for issue where wrapping arithmetic operations
// incorrectly triggered overflow errors during MIR optimization.
// The wrapping_sub, wrapping_add, and wrapping_mul operations
// should never cause compile-time overflow errors, even when
// the operation would overflow for regular arithmetic.

fn main() {
    // These should all compile and run successfully
    assert_eq!(1_u32.wrapping_sub(2_u32), 4294967295);
    assert_eq!(u32::MAX.wrapping_add(1), 0);
    assert_eq!(u32::MAX.wrapping_mul(2), u32::MAX - 1);
    
    // Test with i8 as well
    assert_eq!((-128_i8).wrapping_sub(1), 127);
    assert_eq!(127_i8.wrapping_add(1), -128);
    
    // Test that const evaluation also works
    const WRAP_SUB: u32 = 1_u32.wrapping_sub(2_u32);
    const WRAP_ADD: u32 = u32::MAX.wrapping_add(1);
    const WRAP_MUL: u32 = u32::MAX.wrapping_mul(2);
    
    assert_eq!(WRAP_SUB, 4294967295);
    assert_eq!(WRAP_ADD, 0);
    assert_eq!(WRAP_MUL, u32::MAX - 1);
    
    println!("All wrapping arithmetic tests passed!");
}
