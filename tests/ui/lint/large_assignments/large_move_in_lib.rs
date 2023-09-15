// aux-build:change_box.rs
// build-pass

#![feature(large_assignments)]
#![move_size_limit = "1000"]

fn main() {
    let large_box = Box::new([0; 9999]);
    let _new_large_box = change_box::change_box(large_box);
}
