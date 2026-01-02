The feature gate for the issue is `#![feature(on_broken_pipe)]`.

It grants access to `std::io::OnBrokenPipe` so that an [externally implementable item](https://github.com/rust-lang/rust/issues/125418) called `fn on_broken_pipe() -> std::io::OnBrokenPipe` that determines how `BrokenPipe` errors are treated can be overridden.

Tjs

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

/// The standard library will ask this function what to do with `SIGPIPE` before `fn main()` is invoked.
/// Here we tell it to inherit `SIGPIPE` from the parent process, which in practice means `SIG_DFL`.
/// This can also come from an external crate that we link with.
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

### TODO

* Don't require featuree(eii) also.

#### More Info

Please refer to the [unstable book section](https://doc.rust-lang.org/nightly/unstable-book/language-features/unix-sigpipe.html) for more details. In short:

| `#[unix_sigpipe = "..."]` |   Behaviour
|---------------------------|------------------
| `sig_ign`                 | Set `SIGPIPE` handler to `SIG_IGN` before invoking `fn main()`. Default behaviour since 2014.
| `sig_dfl`                 | Set `SIGPIPE` handler to `SIG_DFL` before invoking `fn main()`.
| `inherit`                 | Leave `SIGPIPE` handler untounched before entering `fn main()`.

The problem with the current `SIGPIPE` code in libstd as well as several other aspects of this problem is discussed extensively at these places:
* https://github.com/rust-lang/rust/issues/62569
* https://github.com/rust-lang/rust/issues/46016
* https://rust-lang.zulipchat.com/#narrow/stream/219381-t-libs/topic/Proposal.3A.20First.20step.20towards.20solving.20the.20SIGPIPE.20problem


### Naming convention

The naming follows the convention used by [`#![windows_subsystem = "windows|console"]`](https://doc.rust-lang.org/reference/runtime.html#the-windows_subsystem-attribute) where the values `"windows"` and `"console"` have the same names as the actual [linker flags](https://docs.microsoft.com/en-us/cpp/build/reference/subsystem-specify-subsystem?view=msvc-170): `/SUBSYSTEM:WINDOWS` and `/SUBSYSTEM:CONSOLE`.

The names `sig_ign` and `sig_dfl` comes from the signal handler names `SIG_IGN` and `SIG_DFL`.


### Steps

- [x] Implement the feature:
    - https://github.com/rust-lang/rust/pull/97802
    - https://github.com/rust-lang/miri/pull/2532
    - https://github.com/rust-lang/rust/pull/101077 (made `sigpipe::DEFAULT` distinct)
    - https://github.com/rust-lang/rust/pull/102110 (improved [diagnostics](https://github.com/rust-lang/rust/pull/102110/commits/b17ec43637276773cd331844896fe7071de2475c))
    - https://github.com/rust-lang/rust/pull/106092
    - https://github.com/rust-lang/rust/pull/108980
- [ ] Use the attribute in a broad set of non-test-case places to learn how it works in practice.
    - [x] `#[unix_sigpipe = "sig_dfl"]`
        - https://github.com/rust-lang/rust/pull/102587
        - https://github.com/rust-lang/rust/pull/103495
        - https://github.com/moonrepo/espresso/blob/e3f429b01bfd9a0a8956f11b1bc9120084c42d3c/crates/cli/src/main.rs#L18
        - https://github.com/trinitronx/intro-to-rust-kvstore/blob/2c26260a837c33f193cf26cecf49279675c3a6a3/src/main.rs#L8
    - [ ] `#[unix_sigpipe = "sig_ign"]`
    - [ ] `#[unix_sigpipe = "inherit"]`
- [x] Remove `rustc_driver::set_sigpipe_handler()`
    - https://github.com/rust-lang/rust/pull/103536
- [ ] Add a test for `#[unix_sigpipe = "inherit"]` that test that the disposition is actually inherited, rather than assuming SIG_DFL shall always be inherited.
- [ ] Final comment period (FCP)
   - https://github.com/rust-lang/rust/pull/120832
- [ ] Stabilization PR
    - https://github.com/rust-lang/rust/pull/120832

### Unresolved Questions That Blocks Stabilisation 

- [ ] How can we make it easy to put `fn lang_start()` in an [external crate](https://github.com/rust-lang/rust/issues/97889#issuecomment-2007391597) that can be compiled with stable?
- [ ] We should [rename](https://github.com/rust-lang/rust/pull/120832#issuecomment-1987023484) the attribute and attribute values to things that reflect what they do rather than how they do it.

* `#[unix_sigpipe = "sig_dfl"]`
    * None
* `#[unix_sigpipe = "sig_ign"]`
    - [ ] [currently](https://github.com/rust-lang/rust/pull/121573) child processes get `SIG_IGN`, but arguably they should get `SIG_DFL` since that is what most programs assume, and we explicitly made it that way [before](https://github.com/rust-lang/rust/pull/25784).
        * Note that we can implement this without running code right before `exec`, see https://github.com/rust-lang/rust/pull/121578 for the trick.
    - [ ] Should the current default be implemented with a [noop signal handler](https://github.com/rust-lang/rust/issues/62569#issuecomment-1961586025)?
* `#[unix_sigpipe = "inherit"]`
    - [ ] Is the name clear enough? Maybe rename to `unchanged`?


### Unresolved Questions That Does Not Block Stabilisation 

Because these questions can be resolved incrementally after stabilization.

* [ ] What is the long-term plan with regards to changing the default behaviour with regards to ignoring `SIGPIPE`, if we want to do it at all?
    - https://github.com/rust-lang/rust/issues/62569

### Resolved Questions

* [x] Should we stabilize `sig_dfl` or is `inherit` and `sig_ign` sufficient? </br> **Answer:** There are noteworthy examples of real projects that has opted to use `SIG_DFL` to solve the `BrokenPipe` problem. Notably [rustc itself](https://github.com/rust-lang/rust/blob/b11bf65e4aaa125952b6479a63f36e9e83efc32c/compiler/rustc_driver/src/lib.rs#L445). So if we don't stabilize `sig_dfl`, such projects can't make use of our new attribute. Therefore, we also need to stabilize `sig_dfl`.
* [x] Can and should we alter the `BrokenPipe` error message and make it suggest to use the new attribute? **Answer:** No, because that would mean we would end up giving developer advice to users that can't act on the advice.
* [x] Can we use `MSG_NOSIGNAL` with `send()` etc instead of setting `SIGPIPE` globally? **Answer:** [No](https://github.com/rust-lang/rust/issues/62569#issuecomment-1970019721), because there is no equivalent for `write()`, and it would incur an extra syscall for each write-operation, which is likely to have significant performance drawbacks.

Disclaimer: I have taken the liberty to mark some questions resolved that I find unlikely to be controversial. If you would like me to create a proper discussion ticket for any of the resolved or unresolved questions, please let me know!


### About tracking issues

Tracking issues are used to record the overall progress of implementation. They are also used as hubs connecting to other relevant issues, e.g., bugs or open design questions. A tracking issue is however *not* meant for large scale discussion, questions, or bug reports about a feature. Instead, open a dedicated issue for the specific matter and add the relevant feature gate label.

@rustbot label +T-libs