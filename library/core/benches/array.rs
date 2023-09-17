//#![allow(large_assignments)]

use test::black_box;
use test::Bencher;

#[bench]
fn map_256byte_8byte_256(b: &mut Bencher) {
    let arr = [0u64; 25600];
    b.iter(|| black_box(arr).map(|_| black_box(0u64)));
}
