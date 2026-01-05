//@ aux-crate: sigpipe_utils=sigpipe-utils.rs

#![feature(on_broken_pipe)]

#[std::io::on_broken_pipe]
fn inherit_on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Inherit
}

fn main() {
    sigpipe_utils::assert_sigpipe_handler(sigpipe_utils::SignalHandler::Ignore);
}
