#!/usr/bin/env bash

RUSTC=/home/martin/src/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc

$RUSTC \
    --crate-name=foo \
    --crate-type=rlib \
    -Cmetadata=v1 \
    -o libfoo_v1.rlib \
    foo-v1.rs

$RUSTC \
    --crate-name=foo \
    --crate-type=rlib \
    -Cmetadata=v2 \
    -o libfoo_v2.rlib \
    foo-v2.rs

$RUSTC \
    --crate-name=re_export_foo \
    --crate-type=rlib \
    --edition 2018 \
    --extern foo=libfoo_v2.rlib \
    -o libre_export_foo.rlib \
    re-export-foo.rs

# RUSTC_LOG=debug \
$RUSTC \
    --edition 2018 \
    --extern foo=libfoo_v1.rlib \
    --extern re_export_foo=libre_export_foo.rlib \
    -L . \
    --emit=metadata \
    main.rs
