//@ run-pass
//@ check-run-results
//@ normalize-stdout: "0x[0-9a-fA-F]+" -> "$$HEX"

use std::fmt::Display;

fn main() {
    let plain = &mut 777;
    println!("*mut i32: {:?}", plain as *mut i32);
    println!("*const i32: {:?}", plain as *const i32);

    let slice = &mut [1, 2, 3][..];
    println!("*mut [i32]: {:?}", slice as *mut [i32]);
    println!("*const [i32]: {:?}", slice as *const [i32]);

    let vtable = &mut 999 as &mut dyn Display;
    println!("*mut dyn Display: {:?}", vtable as *mut dyn Display);
    println!("*const dyn Display: {:?}", vtable as *const dyn Display);
}
