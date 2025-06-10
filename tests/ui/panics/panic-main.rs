//@ revisions: abort-zero abort-one abort-full unwind-zero unwind-one unwind-full

//@[abort-zero] exec-env: RUST_BACKTRACE=0
//@[abort-zero] compile-flags: -Cpanic=abort
//@[abort-zero] no-prefer-dynamic

//@[abort-one] exec-env: RUST_BACKTRACE=1
//@[abort-one] compile-flags: -Cpanic=abort
//@[abort-one] no-prefer-dynamic

//@[abort-full] exec-env: RUST_BACKTRACE=full
//@[abort-full] compile-flags: -Cpanic=abort
//@[abort-full] no-prefer-dynamic

//@[unwind-zero] exec-env: RUST_BACKTRACE=0

//@[unwind-one] exec-env: RUST_BACKTRACE=1

//@[unwind-full] exec-env: RUST_BACKTRACE=full

//@ no-prefer-dynamic
//@ run-fail
//@ error-pattern:moop
//@ needs-subprocess

fn main() {
    panic!("moop");
}
