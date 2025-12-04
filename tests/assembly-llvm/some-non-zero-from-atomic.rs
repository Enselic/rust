//! Regression test for <https://github.com/rust-lang/rust/issues/60044>.
//@ assembly-output: emit-asm
//@ only-x86_64
//@ compile-flags: -O

#![crate_type = "lib"]

use std::num::NonZeroUsize;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

pub static X: AtomicUsize = AtomicUsize::new(1);

pub unsafe fn get() -> Option<NonZeroUsize> {
    let x = X.load(Relaxed);
    Some(NonZeroUsize::new_unchecked(x))
}

pub unsafe fn get2() -> usize {
    match get() {
        Some(x) => x.get(),
        None => unreachable!(), // not optimized out
    }
}

// CHECK: nordh
