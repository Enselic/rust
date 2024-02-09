#![feature(unix_sigpipe)]

#[unix_sigpipe = "sig_dfl"]
#[unix_sigpipe = "sig_ign"] //~ error: multiple `unix_sigpipe` attributes
fn main() {}
