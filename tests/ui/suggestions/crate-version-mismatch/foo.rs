//@ aux-crate: re_export_dependency=re_export_dependency.rs
//@ aux-crate: dependency=dependency_v1.rs
//@ no-prefer-dynamic

struct MainStruct;

impl From<MainStruct> for dependency::DependencyStruct {
    fn from(value: MainStruct) -> Self {
        dependency::DependencyStruct
    }
}

fn main() {
    re_export_dependency::use_into(MainStruct);
}
