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

    let input_day10 = ArcStr::from(include_str!("../input/2024/day10.txt"));

    let input_day11 = ArcStr::from(include_str!("../input/2024/day11.txt"));

    let input_day13 = ArcStr::from(include_str!("../input/2024/day13.txt"));

    let input_day14 = ArcStr::from(include_str!("../input/2024/day14.txt"));

    let input_day15 = ArcStr::from(include_str!("../input/2024/day15.txt"));

    let input_day17 = ArcStr::from(include_str!("../input/2024/day17.txt"));

    let input_day19 = ArcStr::from(include_str!("../input/2024/day19.txt"));

    let input_day2 = ArcStr::from(include_str!("../input/2024/day2.txt"));

    let input_day21 = ArcStr::from(include_str!("../input/2024/day21.txt"));

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
    let mut group = c.benchmark_group("Day10 - Part1");

    {
        let runner = Factory::day10_part1_d10p1(input_day10.clone())
            .expect("failed to generate input for d10p1");
        group.bench_function("d10p1", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day10 - Part2");

    {
        let runner = Factory::day10_part2_d10p2(input_day10.clone())
            .expect("failed to generate input for d10p2");
        group.bench_function("d10p2", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day11 - Part1");

    {
        let runner = Factory::day11_part1_d11p1base(input_day11.clone())
            .expect("failed to generate input for d11p1base");
        group.bench_function("d11p1base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day11 - Part1");

    {
        let runner = Factory::day11_part1_d11p1selected(input_day11.clone())
            .expect("failed to generate input for d11p1selected");
        group.bench_function("d11p1selected", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day11 - Part2");

    {
        let runner = Factory::day11_part2_d11p2base(input_day11.clone())
            .expect("failed to generate input for d11p2base");
        group.bench_function("d11p2base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day11 - Part2");

    {
        let runner = Factory::day11_part2_d11p2selected(input_day11.clone())
            .expect("failed to generate input for d11p2selected");
        group.bench_function("d11p2selected", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day13 - Part1");

    {
        let runner = Factory::day13_part1_d13p1(input_day13.clone())
            .expect("failed to generate input for d13p1");
        group.bench_function("d13p1", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day13 - Part1");

    {
        let runner = Factory::day13_part1_d13p1simd(input_day13.clone())
            .expect("failed to generate input for d13p1simd");
        group.bench_function("d13p1simd", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day13 - Part2");

    {
        let runner = Factory::day13_part2_d13p2(input_day13.clone())
            .expect("failed to generate input for d13p2");
        group.bench_function("d13p2", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day13 - Part2");

    {
        let runner = Factory::day13_part2_d13p2simd(input_day13.clone())
            .expect("failed to generate input for d13p2simd");
        group.bench_function("d13p2simd", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day14 - Part1");

    {
        let runner = Factory::day14_part1_d14p1(input_day14.clone())
            .expect("failed to generate input for d14p1");
        group.bench_function("d14p1", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day14 - Part2");

    {
        let runner = Factory::day14_part2_d14p2autovec(input_day14.clone())
            .expect("failed to generate input for d14p2autovec");
        group.bench_function("d14p2autovec", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day14 - Part2");

    {
        let runner = Factory::day14_part2_d14p2autovec2(input_day14.clone())
            .expect("failed to generate input for d14p2autovec2");
        group.bench_function("d14p2autovec2", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day14 - Part2");

    {
        let runner = Factory::day14_part2_d14p2base(input_day14.clone())
            .expect("failed to generate input for d14p2base");
        group.bench_function("d14p2base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day14 - Part2");

    {
        let runner = Factory::day14_part2_d14p2official(input_day14.clone())
            .expect("failed to generate input for d14p2official");
        group.bench_function("d14p2official", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day14 - Part2");

    {
        let runner = Factory::day14_part2_d14p2simd(input_day14.clone())
            .expect("failed to generate input for d14p2simd");
        group.bench_function("d14p2simd", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day15 - Part1");

    {
        let runner = Factory::day15_part1_d15p1(input_day15.clone())
            .expect("failed to generate input for d15p1");
        group.bench_function("d15p1", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day15 - Part2");

    {
        let runner = Factory::day15_part2_d15p2(input_day15.clone())
            .expect("failed to generate input for d15p2");
        group.bench_function("d15p2", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day17 - Part1");

    {
        let runner = Factory::day17_part1_d17p1base(input_day17.clone())
            .expect("failed to generate input for d17p1base");
        group.bench_function("d17p1base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day17 - Part2");

    {
        let runner = Factory::day17_part2_d17p2base(input_day17.clone())
            .expect("failed to generate input for d17p2base");
        group.bench_function("d17p2base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day19 - Part1");

    {
        let runner = Factory::day19_part1_day19_part1base(input_day19.clone())
            .expect("failed to generate input for day19_part1base");
        group.bench_function("day19_part1base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day19 - Part1");

    {
        let runner = Factory::day19_part1_day19_part1trie(input_day19.clone())
            .expect("failed to generate input for day19_part1trie");
        group.bench_function("day19_part1trie", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day19 - Part2");

    {
        let runner = Factory::day19_part2_day19_part2base(input_day19.clone())
            .expect("failed to generate input for day19_part2base");
        group.bench_function("day19_part2base", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day19 - Part2");

    {
        let runner = Factory::day19_part2_day19_part2trie(input_day19.clone())
            .expect("failed to generate input for day19_part2trie");
        group.bench_function("day19_part2trie", move |b| b.iter(|| runner.bench(black_box)));
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
    let mut group = c.benchmark_group("Day21 - Part1");

    {
        let runner = Factory::day21_part1_d21p1(input_day21.clone())
            .expect("failed to generate input for d21p1");
        group.bench_function("d21p1", move |b| b.iter(|| runner.bench(black_box)));
    }

    group.finish();
    let mut group = c.benchmark_group("Day21 - Part2");

    {
        let runner = Factory::day21_part2_d21p2(input_day21.clone())
            .expect("failed to generate input for d21p2");
        group.bench_function("d21p2", move |b| b.iter(|| runner.bench(black_box)));
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
    let input_day10 = ArcStr::from(include_str!("../input/2024/day10.txt"));
    let input_day11 = ArcStr::from(include_str!("../input/2024/day11.txt"));
    let input_day13 = ArcStr::from(include_str!("../input/2024/day13.txt"));
    let input_day14 = ArcStr::from(include_str!("../input/2024/day14.txt"));
    let input_day15 = ArcStr::from(include_str!("../input/2024/day15.txt"));
    let input_day17 = ArcStr::from(include_str!("../input/2024/day17.txt"));
    let input_day19 = ArcStr::from(include_str!("../input/2024/day19.txt"));
    let input_day2 = ArcStr::from(include_str!("../input/2024/day2.txt"));
    let input_day21 = ArcStr::from(include_str!("../input/2024/day21.txt"));
    let input_day3 = ArcStr::from(include_str!("../input/2024/day3.txt"));
    let input_day7 = ArcStr::from(include_str!("../input/2024/day7.txt"));
    let input_day8 = ArcStr::from(include_str!("../input/2024/day8.txt"));
    let input_day9 = ArcStr::from(include_str!("../input/2024/day9.txt"));


}

criterion_group!(benches, aoc_benchmark);
criterion_main!(benches);
