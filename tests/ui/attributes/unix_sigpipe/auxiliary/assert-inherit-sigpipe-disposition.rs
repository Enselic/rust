#![feature(unix_sigpipe, rustc_private)]

extern crate libc;

#[unix_sigpipe = "inherit"]
fn main() {
    let arg1 = std::env::args()
        .skip(1)
        .next()
        .expect("Must pass SIG_IGN or SIG_DFL as first arg");

    let expected = match arg1.as_str() {
        "SIG_IGN" => libc::SIG_IGN,
        "SIG_DFL" => libc::SIG_DFL,
        arg => panic!("Must pass SIG_IGN or SIG_DFL as first arg. Got: {}", arg),
    };

    let actual = unsafe {
        let mut actual: libc::sigaction = std::mem::zeroed();
        libc::sigaction(libc::SIGPIPE, std::ptr::null(), &mut actual);
        actual.sa_sigaction
    };

    assert_eq!(
        actual, expected,
        "actual and expected SIGPIPE disposition in differs"
    );
}
