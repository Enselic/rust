//@ revisions: abort-zero abort-one abort-full unwind-zero unwind-one unwind-full

//@[unwind-zero] exec-env: RUST_BACKTRACE=0
//@[unwind-zero] compile-flags: -Cpanic=unwind

//@[abort-one] exec-env: RUST_BACKTRACE=1
//@[abort-one] compile-flags: -Cpanic=abort

//@[abort-full] exec-env: RUST_BACKTRACE=full
//@[abort-full] compile-flags: -Cpanic=abort

//@[unwind-zero] exec-env: RUST_BACKTRACE=0
//@[unwind-zero] compile-flags: -Cpanic=unwind

//@[unwind-one] exec-env: RUST_BACKTRACE=1
//@[unwind-one] compile-flags: -Cpanic=unwind

//@[unwind-full] exec-env: RUST_BACKTRACE=full
//@[unwind-full] compile-flags: -Cpanic=unwind

//@ no-prefer-dynamic
//@ run-fail
//@ error-pattern:moop
//@ needs-subprocess

fn main() {
    panic!("moop");
}
