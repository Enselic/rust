#![feature(alloc_error_handler)]
#![no_std]
#![crate_type = "lib"]
#![deny(unsafe_op_in_unsafe_fn)]

#[alloc_error_handler]
unsafe fn oom(_: core::alloc::Layout) -> ! {
    //~^ ERROR call to unsafe function
    loop {}
}
