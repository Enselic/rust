//@ run-fail
//@ check-run-results
//@ error-pattern:moop
//@ needs-subprocess
//@ exec-env:RUST_BACKTRACE=0
//@ exec-env:RUST_BACKTRACE=1
//@ exec-env:RUST_BACKTRACE=full
//@ compile-flags: -Cpanic=abort
//@ compile-flags: -Cpanic=unwind

fn main() {
    panic!("moop");
}


runtime-switch.rs:
runtime-switch.run.stderr:note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
short-ice-remove-middle-frames-2.rs:
short-ice-remove-middle-frames-2.run.stderr:note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
short-ice-remove-middle-frames.rs://@ exec-env:RUST_BACKTRACE=1
short-ice-remove-middle-frames.run.stderr:note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.