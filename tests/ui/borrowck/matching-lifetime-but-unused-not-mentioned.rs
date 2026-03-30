//! Regression test for <https://github.com/rust-lang/rust/issues/113121>.

//@ edition:2018

#![allow(unused_variables)]

fn takes_static<T: 'static>(_: T) {}

// `arg_static` and `upvar_static` are unused and should not be mentioned in the error message.
fn foo<'a>(foo: &'a u8, arg_static: &'static u16) {
    let upvar_static: &'static u16 = &42;
    let the_closure = move || foo;
    takes_static(the_closure); //~ ERROR: borrowed data escapes outside of function
}

fn main() {}
