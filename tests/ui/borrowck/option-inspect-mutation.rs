#[derive(Debug)]
struct A {
    a: u32,
}

fn main() {
    let mut opt = Some(A { a: 123 });
    let ref_mut_opt = opt.as_mut().inspect(|a| {
        a.a += 123;
    });
    dbg!(ref_mut_opt);
}
