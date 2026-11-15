//@ compile-flags: -Z unstable-options
//@ check-pass

#![feature(rustc_private)]
#![feature(default_field_values)]

extern crate rustc_lint;

use rustc_lint::{
    InExternalMacro, LintPass, LintVec, declare_lint, declare_lint_pass, declare_tool_lint,
    impl_lint_pass,
};

declare_lint! {
    pub TEST_LINT,
    Allow,
    "test",
}

declare_tool_lint! {
    pub rustc::TEST_TOOL_LINT,
    Allow,
    "test",
    @in_external_macro = Report
}

declare_tool_lint! {
    pub rustc::TEST_TOOL_LINT_WITH_SUGGESTIONS,
    Allow,
    "test",
    @in_external_macro = ReportAndSuggest
}

declare_tool_lint! {
    pub rustc::TEST_TOOL_LINT_DEFAULT,
    Allow,
    "test"
}

const _: () = {
    assert!(matches!(TEST_TOOL_LINT.in_external_macro, InExternalMacro::Report));
    assert!(matches!(
        TEST_TOOL_LINT_WITH_SUGGESTIONS.in_external_macro,
        InExternalMacro::ReportAndSuggest
    ));
    assert!(matches!(TEST_TOOL_LINT_DEFAULT.in_external_macro, InExternalMacro::DontReport));
};

struct Foo;

struct Bar<'a>(&'a u32);

impl_lint_pass!(Foo => [TEST_LINT]);
impl_lint_pass!(Bar<'_> => [TEST_LINT]);

declare_lint_pass!(Baz => [TEST_LINT]);

fn main() {}
