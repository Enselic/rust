This PR stabilizes `#[unix_sigpipe = "sig_dfl"]`.

The attribute has been available in nightly since `nightly-2022-09-04`.

It has been used by rustc and rustdoc themselves, and also by third party code ([1](https://github.com/moonrepo/espresso/blob/e3f429b01bfd9a0a8956f11b1bc9120084c42d3c/crates/cli/src/main.rs#L18), [2](https://github.com/trinitronx/intro-to-rust-kvstore/blob/2c26260a837c33f193cf26cecf49279675c3a6a3/src/main.rs#L8)).

I have monitored the keyword `sigpipe` on `rust-lang/rust` during all this time, and no problems have been reported that would suggest we can't stabilize this attribute.

## Summary and examples

Everything you need to know about the stabilized attribute should be documented in the reference: TODO link

## Test cases

* https://github.com/rust-lang/rust/tree/master/tests/ui/attributes/unix_sigpipe

## Edge cases

None that I am aware of.

## Unresolved questions

None. See [tracking issues](https://github.com/rust-lang/rust/issues/97889).


Links to the documentations (the PRs we have made in the previous steps).
Any other relevant information.


The resolutions of any unresolved questions if the stabilization is for an RFC.






A summary, showing examples (e.g. code snippets) what is enabled by this feature.
Links to test cases in our test suite regarding this feature and describe the feature's behavior on encountering edge cases.
Links to the documentations (the PRs we have made in the previous steps).
Any other relevant information.
The resolutions of any unresolved questions if the stabilization is for an RFC.



A summary, showing examples (e.g. code snippets) what is enabled by this feature.
Links to test cases in our test suite regarding this feature and describe the feature's behavior on encountering edge cases.
Links to the documentations (the PRs we have made in the previous steps).
Any other relevant information.
The resolutions of any unresolved questions if the stabilization is for an RFC.



