//@ only-linux
//@ ignore-wasm32
//@ ignore-wasm64

use run_make_support::{cwd, diff, rust_lib_name, rustc};

fn main() {
    rustc().input("foo-v1.rs").crate_type("rlib").crate_name("foo").extra_filename("-v1").metadata("-v1").run();
    rustc().input("foo-v2.rs").crate_type("rlib").crate_name("foo").extra_filename("-v2").metadata("-v2").run();
    rustc().input("re-export-foo.rs").crate_type("rlib").edition("2018").extern_("foo", rust_lib_name("foo-v2")).run();

    let out = rustc()
        .input("main.rs")
        .edition("2018")
        .extern_("foo", rust_lib_name("foo-v1"))
        .extern_("re_export_foo", rust_lib_name("re_export_foo"))
        .library_search_path(cwd())
        .ui_testing()
        .remap_path_prefix(cwd(), "/nordholts")
        .run_fail()
        .stderr_utf8();

    diff()
        .expected_file("main.stderr")
        .actual_text("(rustc)", &out)
        .run();
}
