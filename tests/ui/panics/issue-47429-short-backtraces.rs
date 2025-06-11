// Regression test for #47429: short backtraces were not terminating correctly

//@ compile-flags: -O
//@ compile-flags:-Cstrip=none
//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=1

<<<<<<< HEAD
// This is needed to avoid test output differences across std being built with v0 symbols vs legacy
// symbols.
//@ normalize-stderr: "begin_panic::<&str>" -> "begin_panic"
// This variant occurs on macOS with `rust.debuginfo-level = "line-tables-only"` (#133997)
//@ normalize-stderr: " begin_panic<&str>" -> " std::panicking::begin_panic"
// And this is for differences between std with and without debuginfo.
//@ normalize-stderr: "\n +at [^\n]+" -> ""

||||||| 4a216a25d14 (Share inline(never) generics across crates)
// This is needed to avoid test output differences across std being built with v0 symbols vs legacy
// symbols.
//@ normalize-stderr-test: "begin_panic::<&str>" -> "begin_panic"
// And this is for differences between std with and without debuginfo.
//@ normalize-stderr-test: "\n +at [^\n]+" -> ""

=======
>>>>>>> parent of 4a216a25d14 (Share inline(never) generics across crates)
//@ ignore-msvc see #62897 and `backtrace-debuginfo.rs` test
//@ ignore-android FIXME #17520
//@ ignore-openbsd no support for libbacktrace without filename
//@ ignore-fuchsia Backtraces not symbolized
//@ needs-subprocess

// NOTE(eddyb) output differs between symbol mangling schemes
//@ revisions: legacy v0
//@ [legacy] compile-flags: -Zunstable-options -Csymbol-mangling-version=legacy
//@     [v0] compile-flags: -Csymbol-mangling-version=v0

fn main() {
    panic!()
}
