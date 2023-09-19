// Disabling on android for the time being
// See https://github.com/rust-lang/rust/issues/73535#event-3477699747
#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

#[bench] // 65536
fn actual_bench_chain_collect(b: &mut Bencher) {
    bench_chain_collect(b);
}
fn bench_chain_collect(_b: &mut Bencher) {
    let _data = black_box([0u8; 9999]);
}
