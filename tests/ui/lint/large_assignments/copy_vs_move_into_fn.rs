// build-fail

#![feature(large_assignments)]
#![move_size_limit = "1000"]
#![deny(large_assignments)]
#![allow(unused)]

#[derive(Copy, Clone)]
struct CopyableData([u8; 9999]);

// NOTE: Does not derive Copy.
struct PreventCopy(CopyableData);

fn main() {
    let copyable = CopyableData([42; 9999]);

    // Since we copy data into the function the LLVM IR will contain an actual
    // memcpy, so we want the lint to trigger here.
    accept_copy(copyable); //~ ERROR large_assignments

    // Make a second call to prevent even -Zmir-opt-level=3 from changing the
    // argument passing of the first call into a move. With the default
    // -Zmir-opt-level of 1 both calls will perform parameter passing with a
    // copy though, so we expect the lint to trigger also here.
    accept_copy(copyable); //~ ERROR large_assignments

    // Moving data into a function will typically not result in any memcpy in
    // LLVM IR, not even with -Zmir-opt-level=0. So the lint shall not trigger
    // here.
    accept_move(PreventCopy(copyable));
}

fn accept_copy(d: CopyableData) {}

fn accept_move(d: PreventCopy) {}
