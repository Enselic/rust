//! Test that with `-C panic=abort` the backtrace is not cut off, by ensuring
//! that our own function is in the backtrace. Regression test for
//! <https://github.com/rust-lang/rust/issues/81902>.

//@ run-pass
//@ needs-subprocess
//@ compile-flags: -C panic=abort

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        run_test();
    } else {
        this_function_must_be_in_the_backtrace();
    }
}

fn run_test() {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("whatever")
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("this_function_must_be_in_the_backtrace"));
}

fn this_function_must_be_in_the_backtrace() {
    panic!();
}
