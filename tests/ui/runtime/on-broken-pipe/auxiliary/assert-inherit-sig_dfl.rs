//@ aux-crate: sigpipe_utils=sigpipe-utils.rs

#![feature(on_broken_pipe)]
#![feature(extern_item_impls)]
#![feature(rustc_private)]

extern crate libc;

#[std::io::on_broken_pipe]
fn inherit_on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Kill
}

fn main() {
    //sigpipe_utils::assert_sigpipe_handler(sigpipe_utils::SignalHandler::Default);

    // TODO Remove this block
    let actual = unsafe {
        let mut actual: libc::sigaction = std::mem::zeroed();
        libc::sigaction(libc::SIGPIPE, std::ptr::null(), &mut actual);
        actual.sa_sigaction
    };
    let expected = libc::SIG_DFL;
    assert_eq!(actual, expected, "actual {actual} and expected {expected} SIGPIPE disposition differs");
}
