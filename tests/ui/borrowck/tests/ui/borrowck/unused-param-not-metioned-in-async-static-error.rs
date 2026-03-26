//! Regression test for <https://github.com/rust-lang/rust/issues/113121>.

//@ edition:2018

pub fn takes_static<T: 'static>(_: T) {}

// `bar_static` is unused and should not be mentioned in the error message.
fn foo<'foo>(foo: &'foo u8, bar_static: &'static u16) {
    takes_static(move || foo ); //~ ERROR: borrowed data escapes outside of function
}

fn main() {}
