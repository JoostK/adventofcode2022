use criterion::{criterion_group, criterion_main, Criterion};
use day15::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("day15");
    g.bench_function("part1", |b| b.iter(|| part1::run(INPUT, 2_000_000)));
    g.bench_function("part2", |b| b.iter(|| part2::run(INPUT, 4_000_000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
