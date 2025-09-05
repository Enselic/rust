//@ aux-crate: dependency=dependency-v2.rs

extern crate dependency;

pub use dependency::DependencyStruct;

pub fn use_into(_: impl Into<DependencyStruct>) {}
