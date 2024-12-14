use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

type Int = i64;
type Coord = (Int, Int);
type UCoord = (usize, usize);

#[inline(always)]
fn parse_uint(line: &[u8]) -> Int {
    line.iter().fold(0, |acc, b| acc * 10 + (b - b'0') as Int)
}

#[inline(always)]
fn parse_int(line: &[u8]) -> Int {
    if line[0] == b'-' {
        -1 * parse_uint(&line[1..])
    } else {
        parse_uint(line)
    }
}

#[inline(always)]
fn parse_coord(coord: &str) -> Coord {
    let (x, y) = coord.split_once(",").unwrap();
    let x = &x[2..];
    let res = (parse_int(x.as_bytes()), parse_int(y.as_bytes()));
    res
}

#[inline(always)]
fn parse_input(input: &str) -> impl Iterator<Item = (Coord, Coord)> + '_ {
    input.trim_ascii_end().lines().map(|line| {
        let (start, velocity) = line.split_once(" ").unwrap();
        let res = (parse_coord(start), parse_coord(velocity));
        res
    })
}

const H: usize = 103;
const HI: Int = H as Int;
const W: usize = 101;
const WI: Int = W as Int;

fn predict(p: Coord, v: Coord, times: Int) -> UCoord {
    (
        (p.0 + v.0 * times).rem_euclid(WI) as usize,
        (p.1 + v.1 * times).rem_euclid(HI) as usize,
    )
}

#[aoc(day14, part1, d14p1)]
fn part1(input: &str) -> u32 {
    let times = 100;
    let mut quadrants = [0u64; 4];
    for (p, v) in parse_input(input) {
        let (x, y) = predict(p, v, times);
        if x == W / 2 || y == H / 2 {
            continue;
        }
        let msd = if x < W / 2 { 2 } else { 0 };
        let lsd = if y < H / 2 { 1 } else { 0 };
        quadrants[msd + lsd] += 1;
    }
    quadrants.iter().product::<u64>() as u32
}

#[aoc(day14, part2, d14p2)]
fn part2(input: &str) -> u32 {
    let mut grid = [[0 as Int; W]; H];
    let robots: ArrayVec<(Coord, Coord), 500> = parse_input(input).collect();
    for times in 1..100_000 {
        for &(p, v) in robots.iter() {
            let (x, y) = predict(p, v, times);
            grid[y][x] = times;
            let k = 10;
            if x > k && grid[y][x - k..x].iter().all(|&x| x == times) {
                return times as u32;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day14.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 230435667);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 7709);
    }
}
