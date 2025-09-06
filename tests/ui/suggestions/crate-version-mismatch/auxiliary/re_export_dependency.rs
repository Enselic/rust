//@ edition: 2018
//@ aux-crate: dependency=dependency_v2.rs

pub use dependency;

pub trait Measure<T> {
    fn weight(T) -> usize;
}

pub fn measure(_: impl Measure<dependency::DependencyStruct>) {}
