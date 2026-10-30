//@ check-pass

#![feature(alloc_error_handler)]
#![no_std]
#![crate_type = "lib"]
#![deny(unsafe_op_in_unsafe_fn, unused_unsafe)]

#[alloc_error_handler]
fn result(_: core::alloc::Layout) -> ! {
    loop {}
}

#[expect(unsafe_op_in_unsafe_fn)]
unsafe fn check_user_code(ptr: *const u8) {
    let _ = *ptr;
}
