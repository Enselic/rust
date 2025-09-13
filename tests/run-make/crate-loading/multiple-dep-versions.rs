extern crate dep_2_reexport;
extern crate dependency;
use dep_2_reexport::{OtherType, Trait2, Type, into_type};
use dependency::{Trait, do_something, do_something_trait, do_something_type};

fn main() {
    //do_something(Type);
    //Type.foo();
    //Type::bar();
    //do_something(OtherType);
    //do_something_type(Type);
    //do_something_trait(Box::new(Type) as Box<dyn Trait2>);
    into_type(MainType(42));
}

struct MainType(pub i32);

// Implements `From<MainType>` for `Type` from `dependency-1.rs` but it is for
// `Type` from `dependency-2.rs` that we actually need it. It's easy to mess up
// in real-world projects with more complicated types and dependency graphs. So
// we want the compiler to hint that we have multiple versions of `Type`.
impl From<MainType> for dependency::Type {
    fn from(value: MainType) -> Self {
        dependency::Type(value.0)
    }
}
