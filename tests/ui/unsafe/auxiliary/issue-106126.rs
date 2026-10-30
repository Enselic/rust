#[macro_export]
macro_rules! foo {
    () => {
        unsafe fn __unsf() {}
        unsafe fn __foo() {
            // TODO: Verify
            unsafe {
                __unsf();
            }
        }
    };
}
