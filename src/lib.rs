#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day17;
pub mod day2;
pub mod day3;
pub mod day7;
pub mod day8;
pub mod day9;

aoc_lib!(year = 2024);
