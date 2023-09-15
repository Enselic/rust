// aux-build:change_box.rs
// build-pass

#![feature(large_assignments)]
#![move_size_limit = "1000"]

extern crate change_box;

fn main() {
    let large_box = Box::new([0; 9999]);
    //let _new_large_box = change_box::change_box(large_box);
    let _new_large_box = change_box2(large_box);
}

pub fn change_box2<T>(mut x: Box<T>) -> Box<T> {
    let contents = *x;
    Box::new(contents)
}
