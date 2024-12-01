#[macro_use]
extern crate criterion;
extern crate aoc2024;
extern crate aoc_runner;

use aoc2024::*;
use aoc_runner::ArcStr;
use criterion::Criterion;
use std::fmt::Display;

#[inline]
fn black_box(t: &dyn Display) {
    criterion::black_box(t);
}

fn aoc_benchmark(c: &mut Criterion) {
    
    let input_day1 = ArcStr::from(include_str!("../../../../input/2024/day1.txt"));


    
    let mut group = c.benchmark_group("Day1 - Part1");

    
    {
        let runner = Factory::day1_part1_base(input_day1.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }
    {
        let runner = Factory::day1_part1_bheap(input_day1.clone())
            .expect("failed to generate input for bheap");
        group.bench_function("bheap", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day1 - Part2");

    
    {
        let runner = Factory::day1_part2_base(input_day1.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn input_benchmark(c: &mut Criterion) {
    
    let input_day1 = ArcStr::from(include_str!("../../../../input/2024/day1.txt"));


    
}

criterion_group!(benches, aoc_benchmark);
criterion_main!(benches);
