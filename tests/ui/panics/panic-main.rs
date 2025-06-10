//@ revisions: abort-zero abort-one abort-full panic-zero panic-one panic-full

//@[abort-zero] exec-env: RUST_BACKTRACE=0
//@[abort-zero] compile-flags: -Cpanic=abort

//@[abort-one] exec-env: RUST_BACKTRACE=1
//@[abort-one] compile-flags: -Cpanic=abort

//@[abort-full] exec-env: RUST_BACKTRACE=full
//@[abort-full] compile-flags: -Cpanic=abort

//@[panic-zero] exec-env: RUST_BACKTRACE=0
//@[panic-zero] compile-flags: -Cpanic=panic

//@[panic-one] exec-env: RUST_BACKTRACE=1
//@[panic-one] compile-flags: -Cpanic=panic

//@[panic-full] exec-env: RUST_BACKTRACE=full
//@[panic-full] compile-flags: -Cpanic=abort

//@ run-fail
//@ check-run-results
//@ needs-subprocess

fn main() {
    panic!("moop");
}
