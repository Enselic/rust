// Regression test for https://github.com/rust-lang/rust/issues/127562

fn main() {
    let val = 2;
    let ptr = &raw const val;
    unsafe { *ptr = 3; }
}
