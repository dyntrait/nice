

//! Benchmarks collection trait dispatch and HTTP rate-limit key conversion.

use std::hint::black_box;

use ahash::{AHashMap, AHashSet};
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use indexmap::IndexMap;
use nice_core::collections::{MapLike, SetLike, into_ustr_vec};

const KEY_COUNTS: [usize; 3] = [2, 8, 32];

fn bench_into_ustr_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections/into_ustr_vec");

    for count in KEY_COUNTS {
        let keys = (0..count)
            .map(|index| format!("rate-limit-key-{index}"))
            .collect::<Vec<_>>();
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &keys, |b, keys| {
            b.iter_batched(
                || keys.clone(),
                |keys| black_box(into_ustr_vec(black_box(keys))),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_collection_traits(c: &mut Criterion) {
    let set = (0_u64..32).collect::<AHashSet<_>>();
    let ahash_map = (0_u64..32)
        .map(|key| (key, key))
        .collect::<AHashMap<_, _>>();
    let index_map = (0_u64..32)
        .map(|key| (key, key))
        .collect::<IndexMap<_, _>>();
    let key = 31_u64;

    c.bench_function("collections/AHashSet.contains", |b| {
        b.iter(|| black_box(SetLike::contains(black_box(&set), black_box(&key))));
    });
    c.bench_function("collections/AHashSet.is_empty", |b| {
        b.iter(|| black_box(SetLike::is_empty(black_box(&set))));
    });
    c.bench_function("collections/AHashMap.contains_key", |b| {
        b.iter(|| {
            black_box(MapLike::contains_key(
                black_box(&ahash_map),
                black_box(&key),
            ))
        });
    });
    c.bench_function("collections/AHashMap.is_empty", |b| {
        b.iter(|| black_box(MapLike::is_empty(black_box(&ahash_map))));
    });
    c.bench_function("collections/IndexMap.contains_key", |b| {
        b.iter(|| {
            black_box(MapLike::contains_key(
                black_box(&index_map),
                black_box(&key),
            ))
        });
    });
}

criterion_group!(benches, bench_into_ustr_vec, bench_collection_traits);
criterion_main!(benches);
