//! Regression test for <https://github.com/rust-lang/rust/issues/113121>.

//@ edition:2018

#![allow(unused_variables)]

fn consume<T: 'static>(_: T) {}

fn foo<'a>(
    foo: &'a u8,
    bar: &'static u16, // Unused in closure. Must not be in error.
) {
    let baz: &'static u32 = &42; // Unused in closure. Must not be in error.

    let the_closure = move || foo;
    consume(the_closure); //~ ERROR: borrowed data escapes outside of function
}

fn main() {}
