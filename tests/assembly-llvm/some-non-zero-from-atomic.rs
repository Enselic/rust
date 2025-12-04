//! Regression test for <https://github.com/rust-lang/rust/issues/60044>.

//@ assembly-output: emit-asm
//@ only-x86_64

// We want to check that the None case is optimized away
//@ compile-flags: -O

// Simplify the generated assembly
//@ -Cforce-unwind-tables=no

#![crate_type = "lib"]

use std::num::NonZeroUsize;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

pub static X: AtomicUsize = AtomicUsize::new(1);

#[no_mangle]
pub unsafe fn some_non_zero_from_atomic_get() -> Option<NonZeroUsize> {
    let x = X.load(Relaxed);
    Some(NonZeroUsize::new_unchecked(x))
}

#[no_mangle]
pub unsafe fn some_non_zero_from_atomic_get2() -> usize {
    match some_non_zero_from_atomic_get() {
        Some(x) => x.get(),
        None => unreachable!(), // not optimized out
    }
}

// CHECK: nordh
