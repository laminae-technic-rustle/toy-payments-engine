use criterion::{criterion_group, criterion_main, Criterion};

use lib::bench;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench 20", |b| b.iter(|| bench("bench.csv")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
