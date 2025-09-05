//@ edition: 2018
//@ aux-crate: dependency=dependency_v2.rs

pub use dependency;

pub fn into_dependency_struct(_: impl Into<dependency::DependencyStruct>) {}
