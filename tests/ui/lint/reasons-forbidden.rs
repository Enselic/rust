#![feature(lint_reasons)]

#![forbid(
    unsafe_code,
    //~^ NOTE `forbid` level set here
    //~| NOTE the lint level is defined here
    reason = "our errors & omissions insurance policy doesn't cover unsafe Rust"
)]

use std::ptr;

fn main() {
    let a_billion_dollar_mistake = ptr::null();

    #[allow(unsafe_code)]
    //~^ ERROR allow(unsafe_code) incompatible with previous forbid
    //~| NOTE our errors & omissions insurance policy doesn't cover unsafe Rust
    //~| NOTE overruled by previous forbid
    unsafe {
        //~^ ERROR usage of an `unsafe` block
        *a_billion_dollar_mistake
    }
}
