//@ edition: 2018
//@ aux-crate: dependency=dependency_v2.rs

pub use dependency;

pub fn use_into(_: impl Into<dependency::DependencyStruct>) {}
