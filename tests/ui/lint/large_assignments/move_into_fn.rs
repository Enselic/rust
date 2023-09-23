// build-fail

#![feature(large_assignments)]
#![move_size_limit = "1000"]
#![deny(large_assignments)]
#![allow(unused)]

fn main() {
    one_arg([0; 9999]); //~ ERROR large_assignments
    many_args([0; 9999], true, [0; 9999]);
            //^ ERROR large_assignments
                              //^^ ERROR large_assignments
    let closure = |x: LargeStruct| { //~ ERROR large_assignments
        std::hint::black_box(x); //~ ERROR large_assignments
    };
}

fn one_arg(a: [u8; 9999]) {

}

fn many_args(a: [u8; 9999], b: bool, c: [u8; 9999]) {

 }
