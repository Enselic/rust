# Tracking Issue for externally implementable item `std::io::on_broken_pipe() -> std::io::OnBrokenPipe`

Feature gate: `#![feature(on_broken_pipe)]`

This is a tracking issue for the [externally implementable item](https://github.com/rust-lang/rust/issues/125418) `std::io::on_broken_pipe() -> std::io::OnBrokenPipe` that allows programs to select `SIGPIPE` disposition before `fn main()` is invoked.

Supersedes: https://github.com/rust-lang/rust/issues/97889 (which I will close in the near future)

### Zulip Stream

TODO

### About tracking issues

Tracking issues are used to record the overall progress of implementation.
They are also used as hubs connecting to other relevant issues, e.g., bugs or open design questions.
A tracking issue is however *not* meant for large scale discussion, questions, or bug reports about a feature.
Instead, open a dedicated issue for the specific matter and add the relevant feature gate label.
Discussion comments will get marked as off-topic or deleted.
Repeated discussions on the tracking issue may lead to the tracking issue getting locked.

### Usage

A Rust program that writes a sizeable amount of data to stdout with `println!()` will panic if its output is piped to a short-lived program:

```rust
fn main() {
    loop {
        println!("hello world");
    }
}
```
```bash
% ./main | head
hello world
thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)', library/std/src/io/stdio.rs:1016:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrac
```

This is because `SIGPIPE` is changed to `SIG_IGN` before `fn main()` is invoked. To prevent panicking, a program can override the externally implementable item to request that `SIGPIPE` is _not_ changed before `fn main()` is invoked. Its disposition will remain `SIG_DFL` and the program will be killed without error when the pipe is closed:

```rust
#![feature(on_broken_pipe)]
#![feature(extern_item_impls)]

/// The standard library ask this how to setup `SIGPIPE` before `fn main()` is invoked.
/// Here we tell it to inherit `SIGPIPE` from the parent process, which in practice means `SIG_DFL`.
/// This implememtation can also come from an external crate that we link with.
#[std::io::on_broken_pipe]
fn inherit_on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Inherit
}

fn main() {
    loop {
        println!("hello world");
    }
}
```
```bash
% ./main | head -n 1
hello world
```

#### Public API

```rs
/// This lives in `std::io` and is externally implementable:
fn on_broken_pipe() -> std::io::OnBrokenPipe {
    std::io::OnBrokenPipe::Default
}

```rs
/// Specifies how a program should behave with regards to
/// [`ErrorKind::BrokenPipe`]. Currently only relevant to the `unix` family of
/// operating systems.
#[non_exhaustive]
#[derive(Debug)]
pub enum OnBrokenPipe {
    /// Set `SIGPIPE` to `SIG_IGN` so that pipe I/O problems are reported as
    /// [`ErrorKind::BrokenPipe`] errors. Reset `SIGPIPE` to `SIG_DFL` before
    /// child `exec()`. This has been the default behavior since Rust 1.0.
    Default,
    /// Set `SIGPIPE` to `SIG_IGN` so that pipe I/O problems kills the process.
    /// Don't touch `SIGPIPE` before child `exec()`.
    ///
    /// This is mainly useful when you want programs to terminate when their
    /// output is piped to short-lived programs like `head`.
    Kill,
    /// Set `SIGPIPE` to `SIG_DFL` so that pipe I/O problems are reported as
    /// [`ErrorKind::BrokenPipe`] errors. Don't touch `SIGPIPE` before child
    /// `exec()`.
    Error,
    /// Never touch `SIGPIPE`, including before child `exec()`.
    /// `SIGPIPE` disposition is always inherited from the parent process.
    /// This typically means that programs behave as with [`Self::Kill`].
    Inherit,
}
```

### Steps

- [ ] Implementation
  - [ ] Implement the externally implementable item `std::io::on_broken_pipe() -> std::io::OnBrokenPipe`.
  - [ ] Remove old `-Zon-broken-pipe=...` code.
  - [ ] Make `#![feature(extern_item_impls)]` implicit from `#![feature(on_broken_pipe)]`
  - [ ] Remove `sigpipe: u8` from `fn lang_start()` in `std`.
- [ ] Adjust documentation in relevant places
- [ ] Final comment period (FCP)[^1]
- [ ] Stabilization PR

(Remember to update the `S-tracking-*` label when checking boxes.)

[^1]: https://std-dev-guide.rust-lang.org/feature-lifecycle/stabilization.html

### History

This feature was originally implemented as an attribute `#[unix_sigpipe = "..."]`. It was later changed to a compiler flag `-Zon-broken-pipe=...`. It is now implemented as an externally implementable item `std::io::on_broken_pipe() -> std::io::OnBrokenPipe`.

### Unresolved Questions

- [ ] Can we stabilize `#[feature(on_broken_pipe)]` without stabilizing `#![feature(extern_item_impls)]`?

### Unresolved Questions That Does Not Block Stabilisation 

Because these questions can be resolved after stabilization.

* [ ] What is the long-term plan with regards to changing the default behaviour with regards to ignoring `SIGPIPE`, if we want to do it at all?
    - https://github.com/rust-lang/rust/issues/62569

### Resolved Questions

* [x] Should we stabilize `sig_dfl` or is `inherit` and `sig_ign` sufficient? </br> **Answer:** There are noteworthy examples of real projects that has opted to use `SIG_DFL` to solve the `BrokenPipe` problem. Notably [rustc itself](https://github.com/rust-lang/rust/blob/b11bf65e4aaa125952b6479a63f36e9e83efc32c/compiler/rustc_driver/src/lib.rs#L445). So if we don't stabilize `sig_dfl`, such projects can't make use of our new attribute. Therefore, we also need to stabilize `sig_dfl`.
* [x] Can and should we alter the `BrokenPipe` error message and make it suggest to use the new attribute? **Answer:** No, because that would mean we would end up giving developer advice to users that can't act on the advice.
* [x] Can we use `MSG_NOSIGNAL` with `send()` etc instead of setting `SIGPIPE` globally? **Answer:** [No](https://github.com/rust-lang/rust/issues/62569#issuecomment-1970019721), because there is no equivalent for `write()`, and it would incur an extra syscall for each write-operation, which is likely to have significant performance drawbacks.

Disclaimer: I have taken the liberty to mark some questions resolved that I find unlikely to be controversial. If you would like me to create a proper discussion ticket for any of the resolved or unresolved questions, please let me know!
