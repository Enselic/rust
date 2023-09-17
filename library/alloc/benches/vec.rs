use test::{black_box, Bencher};

const LEN: usize = 16384;

#[bench]
fn bench_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| data.iter().cloned().chain([1]).collect::<Vec<_>>());
}

#[bench]
fn bench_chain_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| data.iter().cloned().chain([1]).chain([2]).collect::<Vec<_>>());
}

#[bench]
fn bench_nest_chain_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        data.iter().cloned().chain([1].iter().chain([2].iter()).cloned()).collect::<Vec<_>>()
    });
}

#[bench]
fn bench_range_map_collect(b: &mut Bencher) {
    b.iter(|| (0..LEN).map(|_| u32::default()).collect::<Vec<_>>());
}

#[bench]
fn bench_chain_extend_ref(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::with_capacity(data.len() + 1);
        v.extend(data.iter().chain([1].iter()));
        v
    });
}

#[bench]
fn bench_chain_extend_value(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::with_capacity(data.len() + 1);
        v.extend(data.iter().cloned().chain(Some(1)));
        v
    });
}

#[bench]
fn bench_rev_1(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::new();
        v.extend(data.iter().rev());
        v
    });
}

#[bench]
fn bench_rev_2(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::with_capacity(data.len());
        v.extend(data.iter().rev());
        v
    });
}

#[bench]
fn bench_map_regular(b: &mut Bencher) {
    let data = black_box([(0, 0); LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::new();
        v.extend(data.iter().map(|t| t.1));
        v
    });
}

#[bench]
fn bench_map_fast(b: &mut Bencher) {
    let data = black_box([(0, 0); LEN]);
    b.iter(|| {
        let mut result: Vec<u32> = Vec::with_capacity(data.len());
        for i in 0..data.len() {
            unsafe {
                *result.as_mut_ptr().add(i) = data[i].0;
                result.set_len(i);
            }
        }
        result
    });
}

fn random_sorted_fill(mut seed: u32, buf: &mut [u32]) {
    let mask = if buf.len() < 8192 {
        0xFF
    } else if buf.len() < 200_000 {
        0xFFFF
    } else {
        0xFFFF_FFFF
    };

    for item in buf.iter_mut() {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;

        *item = seed & mask;
    }

    buf.sort();
}

fn bench_vec_dedup_old(b: &mut Bencher, sz: usize) {
    let mut template = vec![0u32; sz];
    b.bytes = std::mem::size_of_val(template.as_slice()) as u64;
    random_sorted_fill(0x43, &mut template);

    let mut vec = template.clone();
    b.iter(|| {
        let len = {
            let (dedup, _) = vec.partition_dedup();
            dedup.len()
        };
        vec.truncate(len);

        black_box(vec.first());
        vec.clear();
        vec.extend_from_slice(&template);
    });
}

fn bench_vec_dedup_new(b: &mut Bencher, sz: usize) {
    let mut template = vec![0u32; sz];
    b.bytes = std::mem::size_of_val(template.as_slice()) as u64;
    random_sorted_fill(0x43, &mut template);

    let mut vec = template.clone();
    b.iter(|| {
        vec.dedup();
        black_box(vec.first());
        vec.clear();
        vec.extend_from_slice(&template);
    });
}

#[bench]
fn bench_dedup_old_100(b: &mut Bencher) {
    bench_vec_dedup_old(b, 100);
}
#[bench]
fn bench_dedup_new_100(b: &mut Bencher) {
    bench_vec_dedup_new(b, 100);
}

#[bench]
fn bench_dedup_old_1000(b: &mut Bencher) {
    bench_vec_dedup_old(b, 1000);
}
#[bench]
fn bench_dedup_new_1000(b: &mut Bencher) {
    bench_vec_dedup_new(b, 1000);
}

#[bench]
fn bench_dedup_old_10000(b: &mut Bencher) {
    bench_vec_dedup_old(b, 10000);
}
#[bench]
fn bench_dedup_new_10000(b: &mut Bencher) {
    bench_vec_dedup_new(b, 10000);
}

#[bench]
fn bench_dedup_old_100000(b: &mut Bencher) {
    bench_vec_dedup_old(b, 100000);
}
#[bench]
fn bench_dedup_new_100000(b: &mut Bencher) {
    bench_vec_dedup_new(b, 100000);
}

#[bench]
fn bench_flat_map_collect(b: &mut Bencher) {
    let v = vec![777u32; 500000];
    b.iter(|| v.iter().flat_map(|color| color.rotate_left(8).to_be_bytes()).collect::<Vec<_>>());
}

/// Reference benchmark that `retain` has to compete with.
#[bench]
fn bench_retain_iter_100000(b: &mut Bencher) {
    let mut v = Vec::with_capacity(100000);

    b.iter(|| {
        let mut tmp = std::mem::take(&mut v);
        tmp.clear();
        tmp.extend(black_box(1..=100000));
        v = tmp.into_iter().filter(|x| x & 1 == 0).collect();
    });
}

#[bench]
fn bench_retain_100000(b: &mut Bencher) {
    let mut v = Vec::with_capacity(100000);

    b.iter(|| {
        v.clear();
        v.extend(black_box(1..=100000));
        v.retain(|x| x & 1 == 0)
    });
}

#[bench]
fn bench_retain_whole_100000(b: &mut Bencher) {
    let mut v = black_box(vec![826u32; 100000]);
    b.iter(|| v.retain(|x| *x == 826u32));
}

#[bench]
fn bench_next_chunk(b: &mut Bencher) {
    let v = vec![13u8; 2048];

    b.iter(|| {
        const CHUNK: usize = 8;

        let mut sum = [0u32; CHUNK];
        let mut iter = black_box(v.clone()).into_iter();

        while let Ok(chunk) = iter.next_chunk::<CHUNK>() {
            for i in 0..CHUNK {
                sum[i] += chunk[i] as u32;
            }
        }

        sum
    })
}
