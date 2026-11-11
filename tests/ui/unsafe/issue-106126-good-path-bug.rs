// Regression test for #106126.

// This used to be check-pass, but had to be changed to check-fail, since
// changing the test to make it check-pass, stops it from catching regressions.
//@ check-fail

//@ aux-build:issue-106126.rs

#![deny(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate issue_106126;

foo!();

fn main() {}
