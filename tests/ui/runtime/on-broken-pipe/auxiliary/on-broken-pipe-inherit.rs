#![feature(extern_item_impls)]
#![feature(on_broken_pipe)]

#[eii(on_broken_pipe)]
fn inherit_on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Inherit
}
