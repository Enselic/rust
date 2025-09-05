//@ aux-crate: dependency=dependency-v2.rs

pub use dependency::DependencyStruct;

fn use_into(_: impl Into<DependencyStruct>) {}
