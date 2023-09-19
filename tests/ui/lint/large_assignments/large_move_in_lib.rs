// --------aux-build:change_box.rs
// build-pass
// compile-flags: -Zinline-mir

#![feature(large_assignments)]
#![move_size_limit = "1000"]

fn main() {
    let _ = std::hint::black_box([0u8; 9999]);
}
