#![feature(start, rustc_private)]

extern crate libc;

// Use #[start] so we don't have a runtime that messes with SIGPIPE.
#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    assert_eq!(argc, 2, "Usage: assert-sigpipe-disposition <SIG_IGN|SIG_DFL>");

    let actual = unsafe {
        let mut actual: libc::sigaction = std::mem::zeroed();
        libc::sigaction(libc::SIGPIPE, std::ptr::null(), &mut actual);
        actual.sa_sigaction
    };

    let expected =
        match unsafe { std::ffi::CStr::from_ptr(*argv.offset(1) as *const i8) }.to_str().unwrap() {
            "ignore" => libc::SIG_IGN,
            "default" => libc::SIG_DFL,
            _ => panic!("expected 'ignore' or 'default'"),
        };

    assert_eq!(actual, expected, "actual and expected SIGPIPE disposition differs");

    return 0;
}
