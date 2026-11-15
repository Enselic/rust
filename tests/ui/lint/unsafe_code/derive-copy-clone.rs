//@ check-pass
//@ compile-flags: --crate-type=lib

#![no_std]
#![forbid(unsafe_code)]

#[derive(Copy, Clone)]
pub struct Value;
