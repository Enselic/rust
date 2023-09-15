// aux-build:change_box.rs
// build-pass

#![feature(large_assignments)]
#![move_size_limit = "1000"]

extern crate change_box;

fn main() {
    let cell = std::cell::UnsafeCell::new([0; 9999]);
    let foo = std::hint::black_box([0; 9999]);
    let large_box = Box::new([0; 9999]);
    let _new_large_box = change_box::change_box(large_box);
}
