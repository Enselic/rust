//@ aux-crate: re_export_dependency_v2=re-export-dependency-v2.rs
//@ aux-crate: dependency=dependency-v1.rs

extern crate re_export_dependency_v2;
extern crate dependency;

use dependency::DependencyStruct;

struct MainStruct;

impl From<MainStruct> for DependencyStruct {
    fn from(value: MainStruct) -> Self {
        DependencyStruct
    }
}

fn main() {
    re_export_dependency_v2::use_into(MainStruct);
}
