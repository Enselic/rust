//@ revisions: default
//@ revisions2: default sig_dfl sig_ign
//@ run-pass
//@ ignore-emscripten
//@ ignore-horizon
//@ aux-build:sigpipe-utils.rs

// Checks the signal disposition of `SIGPIPE`` in child processes. Without any
// unix_sigpipe attribute, `SIG_IGN` is the default. But there is a difference
// in how `SIGPIPE` is treated in child processes with and without the
// attribute. Search for `unix_sigpipe_attr_specified()` in the code base to
// learn more.

#![feature(rustc_private)]
#![allow(dead_code)]

#![cfg_attr(any(sig_dfl, sig_ign), feature(unix_sigpipe))]

mod sigpipe_utils {
    extern crate libc;

    /// So tests don't have to bring libc in scope themselves
    pub enum SignalHandler {
        Ignore,
        Default,
    }

    /// Helper to assert that [`libc::SIGPIPE`] has the expected signal handler.
    pub fn assert_sigpipe_handler(expected_handler: SignalHandler) {
        #[cfg(unix)]
        #[cfg(not(any(
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "horizon",
            target_os = "android",
        )))]
        {
            let prev = unsafe { libc::signal(libc::SIGPIPE, libc::SIG_IGN) };

            let expected = match expected_handler {
                SignalHandler::Ignore => libc::SIG_IGN,
                SignalHandler::Default => libc::SIG_DFL,
            };
            assert_eq!(prev, expected, "expected sigpipe value matches actual value");

            // Unlikely to matter, but restore the old value anyway
            unsafe {
                libc::signal(libc::SIGPIPE, prev);
            };
        }
    }
}

#[cfg_attr(sig_dfl, unix_sigpipe = "sig_dfl")]
#[cfg_attr(sig_ign, unix_sigpipe = "sig_ign")]
fn main() {
    let mut args = std::env::args();
    let me = args.next().unwrap();
    let is_child = args.next().is_some();

    if is_child {
        #[cfg(any(default, sig_ign))]
        let expected = sigpipe_utils::SignalHandler::Ignore;
        #[cfg(sig_dfl)]
        let expected = sigpipe_utils::SignalHandler::Default;
        sigpipe_utils::assert_sigpipe_handler(expected);
    } else {
        assert!(std::process::Command::new(&me).arg("child").status().unwrap().success());
    }
}
