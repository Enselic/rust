//! Partly a regression test for <https://github.com/rust-lang/rust/issues/113121>.

//@ edition:2018

pub fn spawn<F: 'static>(_: F) {}

fn foo<'foo>(foo: &'foo u8, bar_static: &'static u16) {
    spawn(async move { std::hint::black_box(foo); });
    //~^ ERROR: borrowed data escapes outside of function
}

fn main() {}
