This PR stabilizes `#[unix_sigpipe = "sig_dfl"]`.

The attribute has been available in nightly since `nightly-2022-09-04`.

It is being used
[rustc](https://github.com/rust-lang/rust/blob/c29082fe7dc6e902169cacbae165562a7e4a1fd6/compiler/rustc/src/main.rs#L37)
and
[rustdoc](https://github.com/rust-lang/rust/blob/c29082fe7dc6e902169cacbae165562a7e4a1fd6/src/tools/rustdoc/main.rs#L3)
themselves, and also by third party code
([1](https://github.com/moonrepo/espresso/blob/e3f429b01bfd9a0a8956f11b1bc9120084c42d3c/crates/cli/src/main.rs#L18),
[2](https://github.com/trinitronx/intro-to-rust-kvstore/blob/2c26260a837c33f193cf26cecf49279675c3a6a3/src/main.rs#L8)).

I have monitored the keyword `sigpipe` on `rust-lang/rust` during all this time,
and no problems have been reported regarding the attribute. To the contrary.
It's infrastructure is used to implement a
[bugfix](https://github.com/rust-lang/rust/pull/101077/files) in a backwards compatible way.

## Summary and examples

Everything you need to know about the stabilized attribute should be documented in the reference: TODO link

## Test cases

* https://github.com/rust-lang/rust/tree/master/tests/ui/attributes/unix_sigpipe

## Edge cases

None that I am aware of.

## Unresolved questions

None. See [tracking issue](https://github.com/rust-lang/rust/issues/97889).

## Worth noting

This does not stabilize `#[unix_sigpipe = "sig_ign"]` nor `#[unix_sigpipe = "inherit"]`. The reason being that we want to minimize risk of problems, and I have seen no usage of these attributes in the wild. Arguably, we could probably stabilize `#[unix_sigpipe = "sig_ign"]`, but I think it is better to wait with that until we need to. Before we change the default (see [here](https://github.com/rust-lang/rust/issues/62569)) we don't need to. Obviously it is a bit of a chicken and egg problem. But let's wait with solving it for now.

## TODO

* Update the unstable book if we merge this.