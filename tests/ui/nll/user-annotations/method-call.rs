// Unit test for the "user substitutions" that are annotated on each
// node.

trait Bazoom<T> {
    fn method<U>(&self, arg: T, arg2: U) { }
}

impl<T, U> Bazoom<U> for T {
}

fn annot_reference_static_lifetime() {
    let a = 22;
    let b = 44;
    let c = 66;
    a.method::<&'static u32>(b,  &c); //~ ERROR
}

fn main() { }
