#![crate_name="change_box"]
#![crate_type = "lib"]

#![feature(large_assignments)]
#![move_size_limit = "1000"]

/// This function is used to test the `large_assignments` lint.
pub fn change_box<T>(mut x: Box<T>) -> Box<T> {
    let contents = *x;
    Box::new(contents)
}

/// An identity function that *__hints__* to the compiler to be maximally pessimistic about what
/// `black_box` could do.
///
/// See [`std::hint::black_box`] for details.
#[inline(always)]
pub fn black_box<T>(dummy: T) -> T {
    std::hint::black_box(dummy)
}
