//@ check-pass
//@ no-prefer-dynamic
//@ aux-crate: no_prefer_dynamic_lib=no_prefer_dynamic_lib.rs

fn main() {
    no_prefer_dynamic_lib::return_42();
}
