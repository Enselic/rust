//@ aux-crate: on_broken_pipe_inherit=on-broken-pipe-inherit.rs

#![feature(extern_item_impls)]
#![feature(on_broken_pipe)]

#[eii(std::io::on_broken_pipe)]
fn inherit_on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Inherit
}
