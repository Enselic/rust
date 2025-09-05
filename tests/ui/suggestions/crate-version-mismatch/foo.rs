//@ aux-crate: re_export_dependency_v2=re-export-dependency-v2.rs
//@ aux-crate: dependency=dependency-v1.rs

use re_export_dependency_v2::DependencyStruct;

struct MainStruct;

impl From<MainStruct> for StructDependencyStruct {
    fn from(value: MainStruct) -> Self {
        DependencyStruct
    }
}

fn main() {}
