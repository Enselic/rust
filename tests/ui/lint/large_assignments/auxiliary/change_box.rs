#![crate_name="change_box"]
#![crate_type = "lib"]

#![feature(large_assignments)]
#![move_size_limit = "1000"]

/// This function is used to test the `large_assignments` lint.
pub fn change_box<T>(mut x: Box<T>) -> Box<T> {
    let contents = *x;
    Box::new(contents)
}
