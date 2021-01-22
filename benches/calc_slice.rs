use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc_slice");
    group.sample_size(50);
    group.bench_function("slice 10", |b| {
        b.iter(|| mult_persistence::calc_slice(black_box(10)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
