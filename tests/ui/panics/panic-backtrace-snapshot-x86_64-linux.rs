//@ revisions: panic-unwind_force-unwind-tables-no_debuginfo-zero_strip-symbols
//@[force-unwind-tables-no_debuginfo-zero_strip-symbols] compile-flags: -Cpanic=unwind -Cforce-unwind-tables=no -Cdebuginfo=0 -Cstrip=symbols

//@ revisions: panic-unwind_force-unwind-tables-no_debuginfo-zero_strip-none
//@[force-unwind-tables-no_debuginfo-zero_strip-none] compile-flags: -Cpanic=unwind -Cforce-unwind-tables=no -Cdebuginfo=0 -Cstrip=none

//@ revisions: panic-abort_force-unwind-tables-no_debuginfo-zero_strip-symbols
//@[force-unwind-tables-no_debuginfo-zero_strip-symbols] compile-flags: -Cpanic=abort -Cforce-unwind-tables=no -Cdebuginfo=0 -Cstrip=symbols

//@ revisions: panic-abort_force-unwind-tables-no_debuginfo-zero_strip-none
//@[force-unwind-tables-no_debuginfo-zero_strip-none] compile-flags: -Cpanic=abort -Cforce-unwind-tables=no -Cdebuginfo=0 -Cstrip=none

//@ revisions: panic-unwind_force-unwind-tables-yes_debuginfo-zero_strip-symbols
//@[force-unwind-tables-yes_debuginfo-zero_strip-symbols] compile-flags: -Cpanic=unwind -Cforce-unwind-tables=yes -Cdebuginfo=0 -Cstrip=symbols

//@ revisions: panic-unwind_force-unwind-tables-yes_debuginfo-zero_strip-none
//@[force-unwind-tables-yes_debuginfo-zero_strip-none] compile-flags: -Cpanic=unwind -Cforce-unwind-tables=yes -Cdebuginfo=0 -Cstrip=none

//@ revisions: panic-abort_force-unwind-tables-yes_debuginfo-zero_strip-symbols
//@[force-unwind-tables-yes_debuginfo-zero_strip-symbols] compile-flags: -Cpanic=abort -Cforce-unwind-tables=yes -Cdebuginfo=0 -Cstrip=symbols

//@ revisions: panic-abort_force-unwind-tables-yes_debuginfo-zero_strip-none
//@[force-unwind-tables-yes_debuginfo-zero_strip-none] compile-flags: -Cpanic=abort -Cforce-unwind-tables=yes -Cdebuginfo=0 -Cstrip=none

//@ only-x86_64
//@ only-linux
//@ check-run-results
//@ run-fail
//@ exec-env:RUST_LIB_BACKTRACE=1
//@ exec-env:RUST_BACKTRACE=1

fn main() {
    panic!("the panic payload");
    //println!("{}", std::backtrace::Backtrace::force_capture());
}
