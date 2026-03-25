//! Partly a regression test for <https://github.com/rust-lang/rust/issues/113121>.

//@ edition:2018

pub fn spawn<F: 'static>(_: F) {}

fn foo(foo: &usize, bar: &'static bool) {
    spawn(async move { std::hint::black_box(foo) });
}
