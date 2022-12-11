use criterion::{criterion_group, criterion_main, Criterion};
use day10::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("day10");
    g.bench_function("part1", |b| b.iter(|| part1::run(INPUT)));
    g.bench_function("part2", |b| b.iter(|| part2::run(INPUT)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
