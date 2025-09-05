//@ aux-crate: dependency=dependency-v2.rs

extern crate dependency;

// pub use dependency;

pub fn use_into(_: impl Into<dependency::DependencyStruct>) {}
