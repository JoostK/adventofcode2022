use criterion::{criterion_group, criterion_main, Criterion};
use day02::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("day02");
    g.bench_function("part1", |b| b.iter(run::<part1::Assignment>));
    g.bench_function("part2", |b| b.iter(run::<part2::Assignment>));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
