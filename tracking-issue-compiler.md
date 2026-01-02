# Tracking Issue for externally implementable item `std::io::on_broken_pipe() -> std::io::OnBrokenPipe`

Feature gate: `#![feature(on_broken_pipe)]`

This is a tracking issue for the [externally implementable item](https://github.com/rust-lang/rust/issues/125418) `std::io::on_broken_pipe() -> std::io::OnBrokenPipe` that allows programs to select `SIGPIPE` disposition before `fn main()` is invoked.

Supersedes: https://github.com/rust-lang/rust/issues/97889 (which I will close in the near future)

### Usage



### Public API and 

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

sha(rust-lang/rust#125418).

<!--
Include a short description of the feature.
-->


<!--
For most library features, it'd be useful to include a summarized version of the public API.
(E.g. just the public function signatures without their doc comments or implementation.)
-->

```rust
// core::magic

pub struct Magic;

impl Magic {
    pub fn magic(self);
}
```

### Steps / History

<!--
For larger features, more steps might be involved.
If the feature is changed later, please add those PRs here as well.
-->

(Remember to update the `S-tracking-*` label when checking boxes.)

- [ ] Implementation: #...
- [ ] Final comment period (FCP)[^1]
- [ ] Stabilization PR

<!--
Once the feature has gone through a few release cycles and there are no
unresolved questions left, the feature might be ready for stabilization.

If this feature didn't go through the RFC process, a final comment period
(FCP) is always needed before stabilization. This works as follows:

A library API team member can kick off the stabilization process, at which point
the rfcbot will ask all the team members to verify they agree with
stabilization. Once enough members agree and there are no concerns, the final
comment period begins: this issue will be marked as such and will be listed
in the next This Week in Rust newsletter. If no blocking concerns are raised in
that period of 10 days, a stabilization PR can be opened by anyone.
-->

### Unresolved Questions

<!--
Include any open questions that need to be answered before the feature can be
stabilised. If multiple (unrelated) big questions come up, it can be a good idea
to open a separate issue for each, to make it easier to keep track of the
discussions.

It's useful to link any relevant discussions and conclusions (whether on GitHub,
Zulip, or the internals forum) here.
-->

- None yet.

[^1]: https://std-dev-guide.rust-lang.org/feature-lifecycle/stabilization.html







TODO: Don't depend on eii feature also.

<!--
NOTE: For library features, please use the "Library Tracking Issue" template instead.

Thank you for creating a tracking issue! 📜 Tracking issues are for tracking a
feature from implementation to stabilisation. Make sure to include the relevant
RFC for the feature if it has one. Otherwise provide a short summary of the
feature and link any relevant PRs or issues, and remove any sections that are
not relevant to the feature.

Remember to add team labels to the tracking issue.
For a language team feature, this would e.g., be `T-lang`.
Such a feature should also be labeled with e.g., `F-my_feature`.
This label is used to associate issues (e.g., bugs and design questions) to the feature.
-->

This is a tracking issue for the RFC "XXX" (rust-lang/rfcs#NNN).
The feature gate for the issue is `#![feature(FFF)]`.

### About tracking issues

Tracking issues are used to record the overall progress of implementation.
They are also used as hubs connecting to other relevant issues, e.g., bugs or open design questions.
A tracking issue is however *not* meant for large scale discussion, questions, or bug reports about a feature.
Instead, open a dedicated issue for the specific matter and add the relevant feature gate label.
Discussion comments will get marked as off-topic or deleted.
Repeated discussions on the tracking issue may lead to the tracking issue getting locked.

### Steps
<!--
Include each step required to complete the feature. Typically this is a PR
implementing a feature, followed by a PR that stabilises the feature. However
for larger features an implementation could be broken up into multiple PRs.
-->

- [ ] Implement the RFC (cc @rust-lang/XXX -- can anyone write up mentoring
      instructions?)
- [ ] Adjust documentation ([see instructions on rustc-dev-guide][doc-guide])
- [ ] Style updates for any new syntax ([nightly-style-procedure])
  - [ ] Style team decision on new formatting
  - [ ] Formatting for new syntax has been added to the [Style Guide]
  - [ ] (non-blocking) Formatting has been implemented in `rustfmt`
- [ ] Stabilization PR ([see instructions on rustc-dev-guide][stabilization-guide])

[stabilization-guide]: https://rustc-dev-guide.rust-lang.org/stabilization_guide.html#stabilization-pr
[doc-guide]: https://rustc-dev-guide.rust-lang.org/stabilization_guide.html#documentation-prs
[nightly-style-procedure]: https://github.com/rust-lang/style-team/blob/main/nightly-style-procedure.md 
[Style Guide]: https://github.com/rust-lang/rust/tree/HEAD/src/doc/style-guide

### Unresolved Questions
<!--
Include any open questions that need to be answered before the feature can be
stabilised.
-->

XXX --- list all the "unresolved questions" found in the RFC to ensure they are
not forgotten

### Implementation history

<!--
Include a list of all the PRs that were involved in implementing the feature.
-->










