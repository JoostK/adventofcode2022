use criterion::{criterion_group, criterion_main, Criterion};
use day01::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("day01");
    g.bench_function("part1", |b| b.iter(run::<part1::Accumulator>));
    g.bench_function("part2", |b| b.iter(run::<part2::Accumulator>));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
