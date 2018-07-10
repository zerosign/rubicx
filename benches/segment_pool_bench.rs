#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn segment_pool_create(c: &mut Criterion) {
    c.bench_functions("segment_pool_create", |b| b.iter(|| 1));
}

criterion_group!(benches, segment_pool_create);
criterion_main!(benches);
