use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;
use std::{
    borrow::BorrowMut,
    collections::{BinaryHeap, VecDeque},
};

struct File {
    size: usize,
    value: u64,
    start: usize,
}

#[derive(Eq, PartialEq)]
struct Gap {
    size: usize,
    start: usize,
}
impl Ord for Gap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort by start position in descending order
        other
            .start
            .cmp(&self.start)
            // Then by size in descending order
            .then(other.size.cmp(&self.size))
    }
}

impl PartialOrd for Gap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const MAX_SIZE: usize = 10_000;
fn parse(input: &str) -> (VecDeque<File>, VecDeque<Gap>) {
    let mut idx = 0;
    let mut filled = 0;
    let mut files = VecDeque::<File>::with_capacity(MAX_SIZE);
    let mut gaps = VecDeque::<Gap>::with_capacity(MAX_SIZE);
    for (i, &chr) in input.trim_ascii_end().as_bytes().iter().enumerate() {
        let size = (chr - b'0') as usize;
        if i % 2 == 0 {
            files.push_back(File {
                size,
                value: idx,
                start: filled,
            });
            idx += 1;
        } else {
            gaps.push_back(Gap {
                size,
                start: filled,
            });
        }
        filled += size;
    }
    (files, gaps)
}

//      return sum(d.value * i for d in files for i in range(d.start, d.start + d.size))
fn score(files: &[File]) -> u64 {
    files
        .iter()
        .flat_map(|f| (f.start..f.start + f.size).map(move |i| f.value * i as u64))
        .sum()
}

#[aoc(day9, part1, d09p1)]
pub fn part1(input: &str) -> u64 {
    let (mut files, mut gaps) = parse(input);
    while let Some(mut gap) = gaps.pop_front() {
        if gap.size == 0 {
            continue;
        }
        if let Some(mut file) = files.pop_back() {
            if file.start + file.size < gap.start {
                files.push_back(file);
                continue;
            }
            if file.size >= gap.size {
                let leftover = file.size - gap.size;
                if leftover > 0 {
                    files.push_back(File {
                        size: leftover,
                        value: file.value,
                        start: file.start,
                    });
                }
                file.size = gap.size;
                file.start = gap.start;
                files.push_front(file);
            } else {
                file.start = gap.start;
                gap.size -= file.size;
                gap.start += file.size;
                files.push_front(file);
                gaps.push_front(gap);
            }
        } else {
            gaps.push_front(gap);
            break;
        }
    }
    score(files.make_contiguous())
}

#[aoc(day9, part2, d09p2)]
pub fn part2(input: &str) -> u64 {
    let (mut files, ogaps) = parse(input);
    let mut gaps = [(); 10].map(|_| BinaryHeap::<Gap>::new());
    for gap in ogaps {
        gaps[gap.size].push(gap);
    }
    let mut moved = ArrayVec::<File, MAX_SIZE>::new();
    while let Some(mut file) = files.pop_back() {
        let earliest = gaps[file.size..]
            .iter()
            .filter(|gd| !gd.is_empty())
            .flat_map(|gd| gd.peek())
            .min_by_key(|gap| gap.start);
        if let Some(earliest) = earliest {
            if earliest.start >= file.start + file.size {
                moved.push(file);
            } else {
                let mut earliest = gaps[earliest.size].pop().unwrap();
                file.start = earliest.start;
                let leftovers = earliest.size - file.size;
                earliest.size = leftovers;
                earliest.start += file.size;
                moved.push(file);
                let gapd = gaps[leftovers].borrow_mut();
                gapd.push(earliest);
            }
        } else {
            files.push_back(file);
        }
    }
    score(moved.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day9.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 6398252054886);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 6415666220005);
    }
}
