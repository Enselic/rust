//@ run-pass
//@ aux-build:sigpipe-utils.rs
//@ only-unix because SIGPIPE is a unix thing

// FIXME: Should not be needed TODO: Create specific issue
//@ no-prefer-dynamic

#![feature(on_broken_pipe)]
#![feature(extern_item_impls)]

#[std::io::on_broken_pipe]
fn inherit_on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Error
}

fn main() {
    extern crate sigpipe_utils;

    // `-Zon-broken-pipe=error` is active, so we expect SIGPIPE to be ignored.
    sigpipe_utils::assert_sigpipe_handler(sigpipe_utils::SignalHandler::Ignore);
}
