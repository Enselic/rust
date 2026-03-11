//! Test that stepping through a simple program with a debugger one line at a
//! time works intuitively, e.g. that `next` takes you to the next source line.
//! Regression test for <https://github.com/rust-lang/rust/issues/33013>.

//@ ignore-aarch64: Doesn't work yet.
//@ ignore-loongarch64: Doesn't work yet.
//@ ignore-riscv64: Doesn't work yet.
//@ ignore-backends: gcc

// Debugger tests need debuginfo
//@ compile-flags: -g

// The `SingleUseConsts` MIR pass _improves_ compile times, so we want to run it even with -O0.
// For variables that 
// FIXME(#128945): SingleUseConsts shouldn't need to be disabled.
//@ revisions: default-mir-passes no-SingleUseConsts-mir-pass
//@ [no-SingleUseConsts-mir-pass] compile-flags: -Zmir-enable-passes=-SingleUseConsts

//@ gdb-command: run
// FIXME(#97083): Should we be able to break on initialization of zero-sized types?
// FIXME(#97083): Right now the first breakable line is:
//@ gdb-check:   let mut c = 27;
//@ gdb-command: next
//@ gdb-check:   let d = c = 99;
//@ gdb-command: next
//@ [no-SingleUseConsts-mir-pass] gdb-check:   let e = "hi bob";
//@ [no-SingleUseConsts-mir-pass] gdb-command: next
//@ [no-SingleUseConsts-mir-pass] gdb-check:   let f = b"hi bob";
//@ [no-SingleUseConsts-mir-pass] gdb-command: next
//@ [no-SingleUseConsts-mir-pass] gdb-check:   let g = b'9';
//@ [no-SingleUseConsts-mir-pass] gdb-command: next
//@ gdb-check:   let h = ["whatever"; 8];
//@ gdb-command: next
//@ gdb-check:   let i = [1,2,3,4];
//@ gdb-command: next
//@ gdb-check:   let j = (23, "hi");
//@ gdb-command: next
//@ gdb-check:   let k = 2..3;
//@ gdb-command: next
//@ gdb-check:   let l = &i[k];
//@ gdb-command: next
//@ gdb-check:   let m: *const() = &a;

// === LLDB TESTS ==================================================================================

//@ lldb-command:run
// FIXME(#97083): Should we be able to break on initialization of zero-sized types?
// FIXME(#97083): Right now the first breakable line is:
//@ lldb-check:[...]let mut c = 27;[...]
//@ lldb-command:next
//@ lldb-command:frame select
//@ lldb-check:[...]let d = c = 99;[...]
//@ lldb-command:next
//@ [no-SingleUseConsts-mir-pass] lldb-command:frame select
//@ [no-SingleUseConsts-mir-pass] lldb-check:[...]let e = "hi bob";[...]
//@ [no-SingleUseConsts-mir-pass] lldb-command:next
//@ [no-SingleUseConsts-mir-pass] lldb-command:frame select
//@ [no-SingleUseConsts-mir-pass] lldb-check:[...]let f = b"hi bob";[...]
//@ [no-SingleUseConsts-mir-pass] lldb-command:next
//@ [no-SingleUseConsts-mir-pass] lldb-command:frame select
//@ [no-SingleUseConsts-mir-pass] lldb-check:[...]let g = b'9';[...]
//@ [no-SingleUseConsts-mir-pass] lldb-command:next
//@ lldb-command:frame select
//@ lldb-check:[...]let h = ["whatever"; 8];[...]
//@ lldb-command:next
//@ lldb-command:frame select
//@ lldb-check:[...]let i = [1,2,3,4];[...]
//@ lldb-command:next
//@ lldb-command:frame select
//@ lldb-check:[...]let j = (23, "hi");[...]
//@ lldb-command:next
//@ lldb-command:frame select
//@ lldb-check:[...]let k = 2..3;[...]
//@ lldb-command:next
//@ lldb-command:frame select
//@ lldb-check:[...]let l = &i[k];[...]
//@ lldb-command:next
//@ lldb-command:frame select
//@ lldb-check:[...]let m: *const() = &a;[...]

fn main () {
    let a = (); // #break
    let b : [i32; 0] = [];
    let mut c = 27;
    let d = c = 99;
    let e = "hi bob";
    let f = b"hi bob";
    let g = b'9';
    let h = ["whatever"; 8];
    let i = [1,2,3,4];
    let j = (23, "hi");
    let k = 2..3;
    let l = &i[k];
    let m: *const() = &a;
}
