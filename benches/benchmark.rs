use criterion::{black_box, criterion_group, criterion_main, Criterion};

use lib::bench;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("main 20", |b| b.iter(|| bench("transactions.csv")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
