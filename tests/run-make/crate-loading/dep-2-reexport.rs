#![crate_name = "foo"]
#![crate_type = "rlib"]

pub extern crate dependency; // Also re-export the whole crate for more variation
pub use dependency::{Trait2, Type, do_something_trait, do_something_type};
pub struct OtherType;
impl dependency::Trait for OtherType {
    fn foo(&self) {}
    fn bar() {}
}
// This is `Type` from dependency-2.rs
pub fn into_type(_: impl Into<dependency::Type>) {}
