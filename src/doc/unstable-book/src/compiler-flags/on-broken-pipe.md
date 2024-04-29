# `on-broken-pipe`

--------------------

The tracking issue for this feature is: [#97889]

Note: The ui for this feature was previously an attribute named `#[unix_sigpipe = "..."]`.

[#97889]: https://github.com/rust-lang/rust/issues/97889

---

The `-Zon-broken-pipe=...` compiler flag can be used to specify how libstd shall setup `SIGPIPE` on Unix platforms before invoking `fn main()`. This flag is ignored on non-Unix targets.

The flag can be used with three different values or be omitted entirely:

| Flag                       | `SIGPIPE` in this process | `SIGPIPE` action before exec |
| `-Zon-broken-pipe=kill`    | 

There flag are three variants:
* 
* 
* `-Zon-broken-pipe=error`
But also a fourth setting:
* Not used


## `-Zon-broken-pipe=inherit`

Leave `SIGPIPE` untouched before entering `fn main()`. Unless the parent process has changed the default `SIGPIPE` handler from `SIG_DFL` to something else, this will behave the same as `-Zon-broken-pipe=kill`.

## `-Zon-broken-pipe=kill`

Set the `SIGPIPE` handler to `SIG_DFL`. This will result in your program getting killed if it tries to write to a closed pipe. This is normally what you want if your program produces textual output.

### Example

```rust,no_run
fn main() {
    loop {
        println!("hello world");
    }
}
```

```console
$ rustc -Zon-broken-pipe=kill main.rs
$ ./main | head -n1
hello world
```

## `-Zon-broken-pipe=error`

Set the `SIGPIPE` handler to `SIG_IGN` before invoking `fn main()`. This will result in `ErrorKind::BrokenPipe` errors if you program tries to write to a closed pipe. This is normally what you want if you for example write socket servers, socket clients, or pipe peers.

This is what libstd has done by default since 2014. (However, see the note on child processes below.)

### Example

```rust,no_run
fn main() {
    loop {
        println!("hello world");
    }
}
```

```console
$ rustc -Zon-broken-pipe=error main.rs
$ ./main | head -n1
hello world
thread 'main' panicked at library/std/src/io/stdio.rs:1118:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Not used



### Note on child processes

When spawning child processes, the legacy Rust behavior if `-Zon-broken-pipe=...` is not specified is to reset `SIGPIPE` to `SIG_DFL` first.

If `-Zon-broken-pipe=...` is specified, no matter what its value is, the signal disposition of `SIGPIPE` is no longer reset. This means that the child inherits the parent's `SIGPIPE` behavior.
