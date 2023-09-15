#![crate_name="change_box"]
#![crate_type = "lib"]

/// This function is used to test the `large_assignments` lint.
pub fn change_box<T>(mut x: Box<T>) -> Box<T> {
    let contents = Box::<T>::into_inner(x);
    Box::new(contents)
}
