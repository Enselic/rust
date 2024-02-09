This PR stabilizes `#[unix_sigpipe = "sig_dfl"]`.

The `#[unix_sigpipe = "sig_dfl"]` attribute has been available in nightly since
[`nightly-2022-09-04`](https://github.com/rust-lang/rust/pull/97802#issuecomment-1235978689).

It is being used by
[rustc](https://github.com/rust-lang/rust/blob/c29082fe7dc6e902169cacbae165562a7e4a1fd6/compiler/rustc/src/main.rs#L37)
and
[rustdoc](https://github.com/rust-lang/rust/blob/c29082fe7dc6e902169cacbae165562a7e4a1fd6/src/tools/rustdoc/main.rs#L3)
themselves, and also by third party code
([1](https://github.com/moonrepo/espresso/blob/e3f429b01bfd9a0a8956f11b1bc9120084c42d3c/crates/cli/src/main.rs#L18),
[2](https://github.com/trinitronx/intro-to-rust-kvstore/blob/2c26260a837c33f193cf26cecf49279675c3a6a3/src/main.rs#L8)).

I have monitored the keyword `sigpipe` on `rust-lang/rust` during all this time,
and no problems have been reported regarding the attribute. To the contrary.
It's infrastructure was used to implement a
[bugfix](https://github.com/rust-lang/rust/pull/101077/files) in a backwards compatible way.

## Summary and examples

Everything you need to know about the stabilized attribute should be documented in the reference: TODO link

## Test cases

* https://github.com/rust-lang/rust/tree/master/tests/ui/attributes/unix_sigpipe

## Edge cases

None that I am aware of. But it is worth noting that using `#[unix_sigpipe =
"sig_dfl"]` and receiving a `SIGPIPE` means destructors will not run since the
process will be immediately killed. I documented this in The Reference.

## Unresolved questions

None. See [tracking issue](https://github.com/rust-lang/rust/issues/97889).

## Worth noting

This PR does NOT stabilize any other variant of the attribute, namely
`#[unix_sigpipe = "sig_ign"]` and `#[unix_sigpipe = "inherit"]`.

There are many reasons for this:

* Avoids choice paralysis between leaving `SIGPIPE` be the default (`SIG_IGN`) and explicitly setting it to `SIG_IGN` with `#[unix_sigpipe = "sig_ign"]`
* I have found no one using `#[unix_sigpipe = "sig_ign"]` nor `#[unix_sigpipe = "inherit"]` in the wild, so they are much less tested, and there seems to be no desire for them.
* Stabilizing something means committing to its existence for all eternity, so we should not stabilize attributes that are not absolutely needed.

## Potential future work

There is a discussion about changing the default `SIGPIPE` handler. See [here](https://github.com/rust-lang/rust/issues/62569). Changing the default would require us to also stabilize `#[unix_sigpipe = "sig_ign"]`. But doing that and discussing that is out of scope of this PR.

## TODO

* Update the unstable book [page](https://doc.rust-lang.org/beta/unstable-book/language-features/unix-sigpipe.html) of `unix_sigpipe` if we merge this.

## FCP

Since we can't close the [tracking issue](https://github.com/rust-lang/rust/issues/97889) entirely, my hope is that we can do an FCP on this PR rather than on the tracking issue.
