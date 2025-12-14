use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day_template::solution::Solution;
use std::{fs, hint::black_box};

fn load_from_file(path: &str) -> String {
    fs::read_to_string(path).expect("Couldn't load the file")
}

fn bench_solutions(c: &mut Criterion) {
    let input = load_from_file("./data/input");
    let mut group = c.benchmark_group("Solution");

    group.bench_with_input(BenchmarkId::new("part", 1), &input, |b, input| {
        b.iter(|| black_box(Solution::part_1(input)));
    });

    group.finish();
}

criterion_group!(benches, bench_solutions);
criterion_main!(benches);
