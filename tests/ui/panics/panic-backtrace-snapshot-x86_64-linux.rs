//@ revisions: force-unwind-tables-no_debuginfo-no_strip-symbols
//@[force-unwind-tables-no_debuginfo-no_strip-symbols] compile-flags: -Cforce-unwind-tables=no -Cdebuginfo=no -Cstrip=symbols

//@ revisions: force-unwind-tables-no_debuginfo-no_strip-none
//@[force-unwind-tables-no_debuginfo-no_strip-none] compile-flags: -Cforce-unwind-tables=no -Cdebuginfo=no -Cstrip=none

//@ only-x86_64
//@ only-linux
//@ check-run-results
//@ run-fail
//@ exec-env:RUST_BACKTRACE=1

fn main() {
    panic!("the panic payload");
}
