Since https://github.com/rust-lang/rust/pull/150592 is quite complicated I figured it would be good to split it up in smaller pieces that are easier to digest.

With this eii in **library/std/src/io/mod.rs**:
```rs
// in 
#[eii(on_broken_pipe)]
#[unstable(feature = "on_broken_pipe", issue = "150588")]
pub fn on_broken_pipe() -> OnBrokenPipe {
    OnBrokenPipe::BackwardsCompatible
}
```

you currently get this compilation error:

```
error: attribute macro has missing stability attribute
    --> library/std/src/io/mod.rs:2269:1
     |
2269 | #[eii(on_broken_pipe)]
     | ^^^^^^^^^^^^^^^^^^^^--
     | |
     | in this attribute macro expansion
     |
    ::: library/core/src/macros/mod.rs:1899:5
     |
1899 |     pub macro eii($item:item) {
     |     ------------- in this expansion of `#[eii]`
```

and the following expanded code with ` MAGIC_EXTRA_RUSTFLAGS=-Zunpretty=expanded ./x build library/std`:

```rs
const _: () =
    {
        #[on_broken_pipe]
        fn on_broken_pipe() -> OnBrokenPipe {
            OnBrokenPipe::BackwardsCompatible
        }
    };
unsafe extern "Rust" {
    #[unstable(feature = "on_broken_pipe", issue = "150588")]
    #[rustc_eii_extern_item]
    pub safe fn on_broken_pipe()
    -> OnBrokenPipe;
}
#[rustc_builtin_macro(eii_shared_macro)]
#[eii_extern_target(on_broken_pipe)]
pub macro on_broken_pipe { () => {} }
```

With the fix, that error goes away and you get this expanded code instead:

```rs
const _: () =
    {
        #[on_broken_pipe]
        fn on_broken_pipe() -> OnBrokenPipe {
            OnBrokenPipe::BackwardsCompatible
        }
    };
unsafe extern "Rust" {
    #[unstable(feature = "on_broken_pipe", issue = "150588")]
    #[rustc_eii_extern_item]
    pub safe fn on_broken_pipe()
    -> OnBrokenPipe;
}
#[unstable(feature = "on_broken_pipe", issue = "150588")]
#[rustc_builtin_macro(eii_shared_macro)]
#[eii_extern_target(on_broken_pipe)]
pub macro on_broken_pipe { () => {} }
```

So I think we clearly need this part of 