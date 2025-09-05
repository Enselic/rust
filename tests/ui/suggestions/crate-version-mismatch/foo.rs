//@ aux-crate: re_export_dependency=re_export_dependency.rs
//@ aux-crate: dependency=dependency_v1.rs

// The "right" fix is to replace `dependency::DependencyStruct` with `re_export_dependency::dependency::DependencyStruct` but it can be tricky to realize.

struct MainStruct;

impl From<MainStruct> for dependency::DependencyStruct {
    fn from(value: MainStruct) -> Self {
        dependency::DependencyStruct
    }
}

fn main() {
    re_export_dependency::into_dependency_struct(MainStruct);
}
