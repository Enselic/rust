//@ run-pass
//@ aux-build:sigpipe-utils.rs
//@ only-unix because SIGPIPE is a unix thing

// FIXME: Should not be needed TODO: Create specific issue
//@ no-prefer-dynamic

#![feature(on_broken_pipe)]
#![feature(extern_item_impls)]
#![feature(rustc_attrs)]

#[std::io::on_broken_pipe]
fn on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Kill
}

#[rustc_main]
fn rustc_main() {
    extern crate sigpipe_utils;

    // `OnBrokenPipe::Kill` is active, so SIGPIPE handler shall be
    // SIG_DFL. Note that we have a #[rustc_main], but it should still work.
    sigpipe_utils::assert_sigpipe_handler(sigpipe_utils::SignalHandler::Default);
}

// FIXME: Is this test really needed now? Probably not.
