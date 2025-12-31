; ModuleID = 'some_non_zero_from_atomic_optimization.1eee8d13ddb4f2dd-cgu.0'
source_filename = "some_non_zero_from_atomic_optimization.1eee8d13ddb4f2dd-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@_ZN38some_non_zero_from_atomic_optimization1X17hda94d16ae788a7faE = local_unnamed_addr global <{ [8 x i8] }> <{ [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_a500d906b91607583596fa15e63c2ada = private unnamed_addr constant <{ [40 x i8] }> <{ [40 x i8] c"internal error: entered unreachable code" }>, align 1
@alloc_4f62a6d5cdc4665ded610c5a471aea9a = private unnamed_addr constant <{ [95 x i8] }> <{ [95 x i8] c"/home/martin/src/rust-non-zero-opt/tests/codegen-llvm/some-non-zero-from-atomic-optimization.rs" }>, align 1
@alloc_6fd654f6d66ebf46d1a77d032be1f641 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_4f62a6d5cdc4665ded610c5a471aea9a, [16 x i8] c"_\00\00\00\00\00\00\00<\00\00\00\11\00\00\00" }>, align 8

; Function Attrs: mustprogress nofree norecurse nounwind nonlazybind willreturn memory(readwrite, argmem: none, inaccessiblemem: none) uwtable
define noundef i64 @some_non_zero_from_atomic_get() unnamed_addr #0 {
start:
  %0 = load atomic i64, ptr @_ZN38some_non_zero_from_atomic_optimization1X17hda94d16ae788a7faE monotonic, align 8
  ret i64 %0
}

; Function Attrs: nonlazybind uwtable
define noundef i64 @some_non_zero_from_atomic_get2() unnamed_addr #1 {
start:
  %0 = load atomic i64, ptr @_ZN38some_non_zero_from_atomic_optimization1X17hda94d16ae788a7faE monotonic, align 8
  %1 = icmp eq i64 %0, 0
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %start
; call core::panicking::panic
  tail call void @_ZN4core9panicking5panic17h0c72a63a3aba7c48E(ptr noalias noundef nonnull readonly align 1 @alloc_a500d906b91607583596fa15e63c2ada, i64 noundef 40, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc_6fd654f6d66ebf46d1a77d032be1f641) #3
  unreachable

bb3:                                              ; preds = %start
  ret i64 %0
}

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h0c72a63a3aba7c48E(ptr noalias noundef nonnull readonly align 1, i64 noundef, ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #2

attributes #0 = { mustprogress nofree norecurse nounwind nonlazybind willreturn memory(readwrite, argmem: none, inaccessiblemem: none) uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.78.0-nightly (8ace7ea1f 2024-02-07)"}
