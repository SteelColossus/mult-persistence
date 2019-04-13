#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use mult_persistence;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("slice 10", |b| {
        b.iter(|| mult_persistence::calc_slice(black_box(10)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
