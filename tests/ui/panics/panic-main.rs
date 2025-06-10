//@ revisions: zero one full
//@ revisions: abort unwind
//@[zero] exec-env:RUST_BACKTRACE=0
//@[one] exec-env:RUST_BACKTRACE=1
//@[full] exec-env:RUST_BACKTRACE=full
//@[abort] compile-flags: -Cpanic=abort
//@[unwind] compile-flags: -Cpanic=unwind
//@ run-fail
//@ check-run-results
//@ needs-subprocess

fn main() {
    panic!("moop");
}
