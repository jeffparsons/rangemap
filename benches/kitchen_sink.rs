#[macro_use]
extern crate criterion;

use criterion::Criterion;
use rand::prelude::*;
use std::ops::Range;

fn kitchen_sink(kvs: &[(Range<i32>, bool)]) {
    use rangemap::RangeMap;

    let mut range_map: RangeMap<i32, bool> = RangeMap::new();
    for (range, value) in kvs {
        // TODO: Implement removal, and then alternate inserting and removing.
        range_map.insert(range.clone(), value.clone());
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("kitchen sink", |b| {
        let mut rng = thread_rng();
        let kvs: Vec<(Range<i32>, bool)> = (0..1000)
            .map(|_| {
                let start = rng.gen_range(0, 1000);
                // We don't want the ranges to be too big or too small;
                // we're trying to get a healthy combination of overlaps and non-overlaps.
                let end = start + rng.gen_range(0, 100);
                let value: bool = random();
                (start..end, value)
            })
            .collect();
        b.iter(|| kitchen_sink(&kvs))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
