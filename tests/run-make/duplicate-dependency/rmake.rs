//@ only-linux
//@ ignore-wasm32
//@ ignore-wasm64

use run_make_support::{cwd, diff, rust_lib_name, rustc};

fn main() {
    rustc().input("foo-v1.rs").extra_filename("-v1").metadata("-v1").run();
    rustc().input("foo-v2.rs").extra_filename("-v2").metadata("-v2").run();
    rustc().input("re-export-foo.rs").extern_("foo", rust_lib_name("foo-v2")).run();

    let out = rustc()
        .input("main.rs")
        .extern_("foo", rust_lib_name("foo-v1"))
        .extern_("re-export-foo", rust_lib_name("re-export-foo"))
        .ui_testing()
        .remap_path_prefix(cwd(), "/nordholts")
        .run_fail()
        .stderr_utf8();

    diff()
        .expected_file("main.stderr")
        .actual_text("(rustc)", &lines.join("\n"))
        .run();
}
