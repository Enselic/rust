//@ aux-crate: dependency=dependency_v2.rs
//@ no-prefer-dynamic

pub use dependency;

pub fn use_into(_: impl Into<dependency::DependencyStruct>) {}
