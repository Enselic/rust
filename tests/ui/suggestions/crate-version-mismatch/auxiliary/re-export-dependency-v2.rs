//@ aux-crate: dependency=dependency-v2.rs

pub extern crate dependency;

// pub use dependency;

pub fn use_into(_: impl Into<dependency::DependencyStruct>) {}
