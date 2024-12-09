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

    let input_day1 = ArcStr::from(include_str!("../input/2024/day1.txt"));

    let input_day2 = ArcStr::from(include_str!("../input/2024/day2.txt"));

    let input_day3 = ArcStr::from(include_str!("../input/2024/day3.txt"));

    let input_day7 = ArcStr::from(include_str!("../input/2024/day7.txt"));

    let input_day8 = ArcStr::from(include_str!("../input/2024/day8.txt"));

    let input_day9 = ArcStr::from(include_str!("../input/2024/day9.txt"));


    let mut group = c.benchmark_group("Day1 - Part1");

    {
        let runner = Factory::day1_part1_base(input_day1.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day1 - Part1");

    {
        let runner = Factory::day1_part1_d1p1radixsort(input_day1.clone())
            .expect("failed to generate input for d1p1radixsort");
        group.bench_function("d1p1radixsort", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day1 - Part2");

    {
        let runner = Factory::day1_part2_base(input_day1.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day2 - Part1");

    {
        let runner = Factory::day2_part1_base(input_day2.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day2 - Part2");

    {
        let runner = Factory::day2_part2_base(input_day2.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day3 - Part1");

    {
        let runner = Factory::day3_part1_base(input_day3.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day3 - Part1");

    {
        let runner = Factory::day3_part1_regex(input_day3.clone())
            .expect("failed to generate input for regex");
        group.bench_function("regex", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day3 - Part2");

    {
        let runner = Factory::day3_part2_base(input_day3.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day3 - Part2");

    {
        let runner = Factory::day3_part2_regex(input_day3.clone())
            .expect("failed to generate input for regex");
        group.bench_function("regex", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day7 - Part1");

    {
        let runner = Factory::day7_part1_base(input_day7.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day7 - Part1");

    {
        let runner = Factory::day7_part1_recurse(input_day7.clone())
            .expect("failed to generate input for recurse");
        group.bench_function("recurse", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day7 - Part2");

    {
        let runner = Factory::day7_part2_base(input_day7.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day7 - Part2");

    {
        let runner = Factory::day7_part2_recurse(input_day7.clone())
            .expect("failed to generate input for recurse");
        group.bench_function("recurse", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day8 - Part1");

    {
        let runner = Factory::day8_part1_base(input_day8.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day8 - Part2");

    {
        let runner = Factory::day8_part2_base(input_day8.clone())
            .expect("failed to generate input for base");
        group.bench_function("base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day9 - Part1");

    {
        let runner = Factory::day9_part1_d09p1(input_day9.clone())
            .expect("failed to generate input for d09p1");
        group.bench_function("d09p1", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day9 - Part2");

    {
        let runner = Factory::day9_part2_d09p2(input_day9.clone())
            .expect("failed to generate input for d09p2");
        group.bench_function("d09p2", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn input_benchmark(c: &mut Criterion) {
    let input_day1 = ArcStr::from(include_str!("../input/2024/day1.txt"));
    let input_day2 = ArcStr::from(include_str!("../input/2024/day2.txt"));
    let input_day3 = ArcStr::from(include_str!("../input/2024/day3.txt"));
    let input_day7 = ArcStr::from(include_str!("../input/2024/day7.txt"));
    let input_day8 = ArcStr::from(include_str!("../input/2024/day8.txt"));
    let input_day9 = ArcStr::from(include_str!("../input/2024/day9.txt"));


}

criterion_group!(benches, aoc_benchmark);
criterion_main!(benches);
