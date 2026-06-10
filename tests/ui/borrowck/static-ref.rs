fn foo(_: &'static u32) {}

fn main() {
    let x = 5u32;
    foo(&x);
}
