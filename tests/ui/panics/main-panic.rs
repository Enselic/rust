//@ run-fail
//@ error-pattern:thread 'main' panicked at
//@ needs-subprocess

fn main() {
    unsafe {
        let null: *mut u32 = std::ptr::null_mut();
        *null = 42;
    }
}
