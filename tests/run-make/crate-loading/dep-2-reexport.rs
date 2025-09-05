#![crate_name = "foo"]
#![crate_type = "rlib"]

pub extern crate dependency; // Also re-export the whole crate for broader testing
pub use dependency::{Trait2, Type, do_something_trait, do_something_type};
pub struct OtherType;
impl dependency::Trait for OtherType {
    fn foo(&self) {}
    fn bar() {}
}
// This is `Type2` from dependency-2.rs
pub fn into_type2(_: impl Into<dependency::Type2>) {}
