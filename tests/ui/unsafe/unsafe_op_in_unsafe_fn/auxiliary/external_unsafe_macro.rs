pub unsafe fn unsf() {}

#[macro_export]
macro_rules! unsafe_macro {
    () => {
        $crate::unsf()
    };
}

#[macro_export]
macro_rules! deref_pointer_macro {
    ($p:expr) => {
        *$p
    };
}
