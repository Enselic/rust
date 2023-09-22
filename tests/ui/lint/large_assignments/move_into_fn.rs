// build-fail

#![feature(large_assignments)]
#![move_size_limit = "1000"]
#![deny(large_assignments)]
#![allow(unused)]

struct LargeStruct {
    data: [u8; 9999],
}

fn main() {
    let x = LargeStruct { data: [0; 9999] }; //~ ERROR large_assignments
    std::hint::black_box(x); //~ ERROR large_assignments
}
