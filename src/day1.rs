use std::collections::BinaryHeap;

const SIZE: usize = 1000;

#[aoc(day1, part1, base)]
pub fn part1og(input: &str) -> u32 {
    let mut lhs = [0u32; SIZE];
    let mut rhs = [0u32; SIZE];

    for (i, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        lhs[i] = (0..5).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32);
        rhs[i] = (8..13).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32);
    }

    lhs.sort_unstable();
    rhs.sort_unstable();

    (0..SIZE).map(|i| lhs[i].abs_diff(rhs[i])).sum()
}

#[aoc(day1, part1, bheap)]
pub fn part1(input: &str) -> u32 {
    let mut lhs = BinaryHeap::with_capacity(SIZE);
    let mut rhs = BinaryHeap::with_capacity(SIZE);

    for line in input.lines() {
        let bytes = line.as_bytes();
        let l = (0..5).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as usize);
        lhs.push(l);
        let r = (8..13).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as usize);
        rhs.push(r);
    }

    lhs.into_iter()
        .zip(rhs.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum::<usize>() as u32
}

#[aoc(day1, part2, base)]
pub fn part2(input: &str) -> u32 {
    let mut lhs = [0u32; SIZE];
    let mut rhs = [0u8; 100_000];

    for (i, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        lhs[i] = (0..5).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32);
        let val = (8..13).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32) as usize;
        rhs[val] += 1;
    }

    (0..SIZE)
        .map(|i| lhs[i] * (rhs[lhs[i] as usize] as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1og(INPUT), 1646452);
        assert_eq!(part1(INPUT), 1646452);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 23609874);
    }
}
