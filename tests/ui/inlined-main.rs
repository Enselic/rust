//@ run-pass

#[inline(always)]
fn main() {
    unsafe {
        let null: *mut u32 = std::ptr::null_mut();
        *null = 42;
    }
}
