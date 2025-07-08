//! Test that with `-C panic=abort` the backtrace is not cut off by default, by
//! ensuring that our own function is in the backtrace. Regression test for
//! <https://github.com/rust-lang/rust/issues/81902>.

//@ run-pass
//@ needs-subprocess
//@ compile-flags: -C panic=abort -C opt-level=0
//@ no-prefer-dynamic

static NEEDLE: &str = "this_function_must_be_in_the_backtrace"; 
fn this_function_must_be_in_the_backtrace() {
    panic!("create panic backtrace haystack");
}

fn run_test() {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("whatever")
        .env("RUST_BACKTRACE", "full")
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains(NEEDLE), "ERROR: no `{}` in stderr! actual stderr: {}", NEEDLE, stderr);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        run_test();
    } else {
        this_function_must_be_in_the_backtrace();
    }
}
