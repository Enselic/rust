// Issue #128381: Check that we don't suggest changing closure parameter types
// when they are constrained by method signatures like `Option::inspect`.

#[derive(Debug)]
struct A {
    a: u32,
}

fn main() {
    let mut opt = Some(A { a: 123 });
    let ref_mut_opt = opt.as_mut().inspect(|a| {
        a.a += 123;
        //~^ ERROR cannot assign to `a.a`, which is behind a `&` reference
    });
    dbg!(ref_mut_opt);
}
