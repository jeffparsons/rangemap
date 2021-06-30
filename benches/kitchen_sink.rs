#[macro_use]
extern crate criterion;

use criterion::Criterion;
use rand::prelude::*;
use std::ops::Range;

fn kitchen_sink(kvs: &[(Range<i32>, bool)]) {
    use rangemap::RangeMap;

    let mut range_map: RangeMap<i32, bool> = RangeMap::new();
    // Remove every second range.
    let mut remove = false;
    for (range, value) in kvs {
        if remove {
            range_map.remove(range.clone());
        } else {
            range_map.insert(range.clone(), *value);
        }
        remove = !remove;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("kitchen sink", |b| {
        let mut rng = thread_rng();
        let kvs: Vec<(Range<i32>, bool)> = (0..1000)
            .map(|_| {
                let start = rng.gen_range(0..1000);
                let end = start + rng.gen_range(1..100);
                let value: bool = random();
                (start..end, value)
            })
            .collect();
        b.iter(|| kitchen_sink(&kvs))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
