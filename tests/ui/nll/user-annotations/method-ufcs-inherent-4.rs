// Check that inherent methods invoked with `<T>::new` style
// carry their annotations through to NLL in connection with
// method type parameters.

struct A<'a> { x: &'a u32 }

impl<'a> A<'a> {
    fn new<'b, T>(x: &'a u32, y: T) -> Self {
        Self { x }
    }
}

fn foo<'x>() {
    let v = 22;
    let x = <A<'x>>::new::<&'x u32>(&v, &v);
    //~^ ERROR
    //~| ERROR
}

fn main() {}
