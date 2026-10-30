//@ check-pass
//@ edition:2024

#![deny(unsafe_op_in_unsafe_fn, unused_unsafe)]

use std::alloc::System;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {}
