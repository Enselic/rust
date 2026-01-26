//@ no-prefer-dynamic

//! Since this is `no-prefer-dynamic` we expect compiletest to _not_ think this
//! is an .so file.

#![crate_type = "rlib"]

pub fn return_42() -> i32 {
    42
}
