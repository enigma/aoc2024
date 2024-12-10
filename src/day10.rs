use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

const SIZE: usize = 63;

fn parse(input: &str) -> ArrayVec<ArrayVec<u8, SIZE>, SIZE> {
    let mut res = ArrayVec::new();
    for (_y, line) in input.trim_ascii_end().lines().enumerate() {
        let mut row = ArrayVec::new();
        for (_x, &c) in line.as_bytes().iter().enumerate() {
            row.push(c - b'0');
        }
        res.push(row);
    }
    res
}

fn solvep1_rec(
    y: usize,
    x: usize,
    seen: &mut [[bool; SIZE]; SIZE],
    grid: &ArrayVec<ArrayVec<u8, SIZE>, SIZE>,
) -> u64 {
    let me = *unsafe { grid.get_unchecked(y).get_unchecked(x) };
    let seen_mut = unsafe { seen.get_unchecked_mut(y).get_unchecked_mut(x) };
    if *seen_mut {
        return 0;
    }
    *seen_mut = true;
    if me == 9 {
        return 1;
    }
    let side = grid.len();
    (y.saturating_sub(1)..(y + 2).min(side))
        .flat_map(|y| (x.saturating_sub(1)..(x + 2).min(side)).map(move |x| (y, x)))
        .filter(|&(yy, xx)| !(yy == y && xx == x))
        .filter(|&(yy, xx)| yy == y || xx == x)
        .filter(|&(yy, xx)| *unsafe { grid.get_unchecked(yy).get_unchecked(xx) } == me + 1)
        .map(|(yy, xx)| solvep1_rec(yy, xx, seen, grid))
        .sum()
}

fn solvep1(zy: usize, zx: usize, grid: &ArrayVec<ArrayVec<u8, SIZE>, SIZE>) -> u64 {
    let mut seen = [[false; SIZE]; SIZE];
    solvep1_rec(zy, zx, &mut seen, grid)
}

#[aoc(day10, part1, d10p1)]
pub fn part1(input: &str) -> u64 {
    let grid = parse(input);
    let side = grid.len();
    (0..side)
        .flat_map(|y| (0..side).map(move |x| (y, x)))
        .filter(|&(y, x)| *unsafe { grid.get_unchecked(y).get_unchecked(x) } == 0)
        .map(|(zy, zx)| solvep1(zy, zx, &grid))
        .sum()
}

fn solvep2(
    y: usize,
    x: usize,
    grid: &ArrayVec<ArrayVec<u8, SIZE>, SIZE>,
    cache: &mut [[u64; SIZE]; SIZE],
) -> u64 {
    if cache[y][x] != u64::MAX {
        return cache[y][x];
    }
    let me = *unsafe { grid.get_unchecked(y).get_unchecked(x) };
    if me == 9 {
        return 1;
    }
    let side = grid.len();
    let res = (y.saturating_sub(1)..(y + 2).min(side))
        .flat_map(|y| (x.saturating_sub(1)..(x + 2).min(side)).map(move |x| (y, x)))
        .filter(|&(yy, xx)| !(yy == y && xx == x))
        .filter(|&(yy, xx)| yy == y || xx == x)
        .filter(|&(yy, xx)| *unsafe { grid.get_unchecked(yy).get_unchecked(xx) } == me + 1)
        .map(|(yy, xx)| solvep2(yy, xx, grid, cache))
        .sum();
    cache[y][x] = res;
    res
}

#[aoc(day10, part2, d10p2)]
pub fn part2(input: &str) -> u64 {
    let grid = parse(input);
    let side = grid.len();
    let mut cache = [[u64::MAX; SIZE]; SIZE];
    (0..side)
        .flat_map(|y| (0..side).map(move |x| (y, x)))
        .filter(|&(y, x)| *unsafe { grid.get_unchecked(y).get_unchecked(x) } == 0)
        .map(|(zy, zx)| solvep2(zy, zx, &grid, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input/2024/day10.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 778);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1925);
    }
}
