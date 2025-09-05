//@ aux-crate: re_export_dependency_v2=re-export-dependency-v2.rs

struct MainStruct;

impl From<MainStruct> for re_export_dependency_v2::Struct {
    fn from(value: MainStruct) -> Self {
        re_export_dependency_v2::Struct
    }
}

fn main() {

}