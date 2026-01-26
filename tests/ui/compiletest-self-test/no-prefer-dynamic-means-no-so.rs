//@ no-prefer-dynamic
//@ aux-crate: no_prefer_dynamic_print_hello=no_prefer_dynamic_print_hello.rs
//@ check-pass

fn main() {
    no_prefer_dynamic_print_hello::no_prefer_dynamic_print_hello();
}
