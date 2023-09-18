use test::{black_box, Bencher};

const LEN: usize = 16384;

#[bench] // 65536
fn bench_chain_collect(_b: &mut Bencher) {
    let _data = black_box([0; LEN]);
    //b.iter(|| data.iter().cloned().chain([1]).collect::<Vec<_>>());
}



// 131072
#[bench]
fn bench_map_regular(b: &mut Bencher) {
    let data = black_box([(0, 0); LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::new();
        v.extend(data.iter().map(|t| t.1));
        v
    });
}

