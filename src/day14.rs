use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;
use std::simd::cmp::SimdPartialOrd;
use std::simd::prelude::SimdPartialEq;
use std::simd::Simd;

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
pub fn part1(input: &str) -> u32 {
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

#[aoc(day14, part2, d14p2base)]
fn part2base(input: &str) -> u32 {
    let mut grid = [[0 as Int; W]; H];
    let robots: ArrayVec<(Coord, Coord), 500> = parse_input(input).collect();
    for times in 5_000..100_000 {
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

#[aoc(day14, part2, d14p2simd)]
fn part2simd(input: &str) -> u32 {
    let robots: ArrayVec<(Coord, Coord), 500> = parse_input(input).collect();

    const LANE_SIZE: usize = 8;
    let mut px_vecs = Vec::new();
    let mut py_vecs = Vec::new();
    let mut vx_vecs = Vec::new();
    let mut vy_vecs = Vec::new();

    for chunk in (0..robots.len()).step_by(LANE_SIZE) {
        let chunk_size = (robots.len() - chunk).min(LANE_SIZE);
        // Create temporary vectors padded with zeros
        let mut px_chunk = vec![0; LANE_SIZE];
        let mut py_chunk = vec![0; LANE_SIZE];
        let mut vx_chunk = vec![0; LANE_SIZE];
        let mut vy_chunk = vec![0; LANE_SIZE];

        // Fill with actual data
        for (i, robot) in robots[chunk..chunk + chunk_size].iter().enumerate() {
            px_chunk[i] = robot.0 .0;
            py_chunk[i] = robot.0 .1;
            vx_chunk[i] = robot.1 .0;
            vy_chunk[i] = robot.1 .1;
        }

        px_vecs.push(Simd::from_slice(&px_chunk));
        py_vecs.push(Simd::from_slice(&py_chunk));
        vx_vecs.push(Simd::from_slice(&vx_chunk));
        vy_vecs.push(Simd::from_slice(&vy_chunk));
    }

    let wi_vec = Simd::<i64, 8>::splat(WI);
    let hi_vec = Simd::<i64, 8>::splat(HI);
    // for times in 5_000..10_000 {
    //     let times_vec = Simd::<i64, 8>::splat(times);
    //     for i in 0..px_vecs.len() {
    //         let px_vec = &px_vecs[i];
    //         let py_vec = &py_vecs[i];
    //         let vx_vec = &vx_vecs[i];
    //         let vy_vec = &vy_vecs[i];

    //         let x = {
    //             let div = (*px_vec + (*vx_vec * times_vec)) / wi_vec;
    //             let rem = (*px_vec + (*vx_vec * times_vec)) - (div * wi_vec);
    //             rem + (rem.simd_lt(Simd::splat(0)).select(wi_vec, Simd::splat(0)))
    //         };
    //         let y = {
    //             let div = (*py_vec + (*vy_vec * times_vec)) / hi_vec;
    //             let rem = (*py_vec + (*vy_vec * times_vec)) - (div * hi_vec);
    //             rem + (rem.simd_lt(Simd::splat(0)).select(hi_vec, Simd::splat(0)))
    //         };

    //         for j in 0..8 {
    //             let x_val = x[j];
    //             let y_val = y[j];
    //             if x_val >= 0 && x_val < WI && y_val >= 0 && y_val < HI {
    //                 grid[y_val as usize][x_val as usize] = times;
    //             }
    //         }

    //         for row in grid[60..80].iter() {
    //             let row_vec = Simd::<i64, 8>::from_slice(&row[56..64]);
    //             if row_vec.simd_eq(times_vec).all() {
    //                 return times as u32;
    //             }
    //         }
    //     }
    // }
    let mut it_x = 0;
    let mut cols = [0; W];
    'outer_x: loop {
        // px_vecs.chunk_by(8)
        for (px_vec, vx_vec) in px_vecs.iter_mut().zip(vx_vecs.iter()) {
            let div = (*px_vec + *vx_vec) / wi_vec;
            let rem = (*px_vec + *vx_vec) - (div * wi_vec);
            *px_vec = rem + (rem.simd_lt(Simd::splat(0)).select(wi_vec, Simd::splat(0)));
        }
        it_x += 1;
        for &pos_vec in &px_vecs {
            for &pos in pos_vec.as_array().iter() {
                cols[pos as usize] += 1;
                if cols[pos as usize] >= 32 {
                    break 'outer_x;
                }
            }
        }
        cols.fill(0);
    }
    let mut it_y = 0;
    let mut rows = [0; H];
    'outer_y: loop {
        // px_vecs.chunk_by(8)
        for (py_vec, vy_vec) in py_vecs.iter_mut().zip(vy_vecs.iter()) {
            let div = (*py_vec + *vy_vec) / hi_vec;
            let rem = (*py_vec + *vy_vec) - (div * hi_vec);
            *py_vec = rem + (rem.simd_lt(Simd::splat(0)).select(hi_vec, Simd::splat(0)));
        }
        it_y += 1;
        for &pos_vec in &py_vecs {
            for &pos in pos_vec.as_array().iter() {
                rows[pos as usize] += 1;
                if rows[pos as usize] >= 32 {
                    break 'outer_y;
                }
            }
        }
        rows.fill(0);
    }
    (1..W as u32)
        .map(|i| (it_x + i * W as u32) - it_y)
        .find(|n| *n % H as u32 == 0)
        .unwrap()
        + it_y
}

#[aoc(day14, part2, d14p2official)]
pub fn part2(input: &str) -> u32 {
    part2simd(input)
}

#[aoc(day14, part2, d14p2autovec)]
fn part2autovec(input: &str) -> u32 {
    let mut px = ArrayVec::<i64, 500>::new();
    let mut py = ArrayVec::<i64, 500>::new();
    let mut vx = ArrayVec::<i64, 500>::new();
    let mut vy = ArrayVec::<i64, 500>::new();
    for (p, v) in parse_input(input) {
        px.push(p.0);
        py.push(p.1);
        vx.push(v.0);
        vy.push(v.1);
    }

    let mut it_x = 0;
    let mut cols = [0; W + 1];
    'outer_x: loop {
        for (pos, vel) in px.iter_mut().zip(vx.iter()) {
            *pos = (*pos + vel).rem_euclid(WI as _);
        }
        it_x += 1;
        for &pos in &px {
            cols[pos as usize] += 1;
            if cols[pos as usize] >= 32 {
                break 'outer_x;
            }
        }
        cols.fill(0);
    }
    let mut it_y = 0;
    let mut rows = [0; H];
    'outer_y: loop {
        for (pos, vel) in py.iter_mut().zip(vy.iter()) {
            *pos = (*pos + vel).rem_euclid(HI as _);
        }
        it_y += 1;
        for &pos in &py {
            rows[pos as usize] += 1;
            if rows[pos as usize] >= 32 {
                break 'outer_y;
            }
        }
        rows.fill(0);
    }
    (1..W as u32)
        .map(|i| (it_x + i * W as u32) - it_y)
        .find(|n| *n % H as u32 == 0)
        .unwrap()
        + it_y
}

#[aoc(day14, part2, d14p2autovec2)]
fn part2autovec2(input: &str) -> u32 {
    let robots: ArrayVec<(Coord, Coord), 500> = parse_input(input).collect();
    let mut px = robots
        .iter()
        .map(|(p, _)| p.0)
        .collect::<ArrayVec<_, 500>>();
    let mut py = robots
        .iter()
        .map(|(p, _)| p.1)
        .collect::<ArrayVec<_, 500>>();
    let vx = robots
        .iter()
        .map(|(_, v)| v.0)
        .collect::<ArrayVec<_, 500>>();
    let vy = robots
        .iter()
        .map(|(_, v)| v.1)
        .collect::<ArrayVec<_, 500>>();

    let mut it_x = 0;
    let mut cols = [0; W + 1];
    'outer_x: loop {
        for (pos, vel) in px.iter_mut().zip(vx.iter()) {
            *pos = (*pos + vel).rem_euclid(WI as _);
        }
        it_x += 1;
        for &pos in &px {
            cols[pos as usize] += 1;
            if cols[pos as usize] >= 32 {
                break 'outer_x;
            }
        }
        cols.fill(0);
    }
    let mut it_y = 0;
    let mut rows = [0; H + 1];
    'outer_y: loop {
        for (pos, vel) in py.iter_mut().zip(vy.iter()) {
            *pos = (*pos + vel).rem_euclid(HI as _);
        }
        it_y += 1;
        for &pos in &py {
            rows[pos as usize] += 1;
            if rows[pos as usize] >= 32 {
                break 'outer_y;
            }
        }
        rows.fill(0);
    }
    (1..W as u32)
        .map(|i| (it_x + i * W as u32) - it_y)
        .find(|n| *n % H as u32 == 0)
        .unwrap()
        + it_y
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
        assert_eq!(part2base(INPUT), 7709);
        assert_eq!(part2simd(INPUT), 7709);
        assert_eq!(part2autovec(INPUT), 7709);
        assert_eq!(part2autovec2(INPUT), 7709);
    }
}
