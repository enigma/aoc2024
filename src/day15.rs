use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

type Grid = ArrayVec<ArrayVec<u8, 100>, 100>;
type Pos = (usize, usize);
type Dir = (isize, isize);

fn parse1(input: &str) -> (Grid, Pos, Vec<Dir>) {
    let mut grid = Grid::new();
    let mut me = (0, 0);
    let (grid_s, dir_s) = input.split_once("\n\n").unwrap();
    for (y, line) in grid_s.lines().enumerate() {
        let mut row = ArrayVec::new();
        for (x, &c) in line.as_bytes().iter().enumerate() {
            row.push(if c == b'@' {
                me = (y, x);
                b'.'
            } else {
                c
            });
        }
        grid.push(row);
    }
    let dirs = dir_s
        .as_bytes()
        .iter()
        .filter_map(|c| match c {
            b'^' => Some((-1, 0)),
            b'v' => Some((1, 0)),
            b'>' => Some((0, 1)),
            b'<' => Some((0, -1)),
            _ => None,
        })
        .collect();
    (grid, me, dirs)
}

const EMPTY: u8 = b'.';
const WALL: u8 = b'#';
const STONE: u8 = b'O';
const STONE_L: u8 = b'[';
const STONE_R: u8 = b']';

#[aoc(day15, part1, d15p1)]
pub fn part1(input: &str) -> u32 {
    let (mut grid, me, dirs) = parse1(input);
    let height = grid.len();
    let width = grid[0].len();

    let (mut my, mut mx) = (me.0 as isize, me.1 as isize);
    for (dy, dx) in dirs {
        let (mut cy, mut cx) = (my, mx);
        let (mut ny, mut nx);
        loop {
            ny = cy + dy;
            nx = cx + dx;
            if ny < 0 || ny >= height as isize || nx < 0 || nx >= width as isize {
                break;
            }
            if grid[ny as usize][nx as usize] != STONE {
                break;
            }
            cy = ny;
            cx = nx;
        }
        let uny = ny as usize;
        let unx = nx as usize;
        if 0 <= ny && uny < height && 0 <= nx && unx < width && grid[uny][unx] == EMPTY {
            if grid[(my + dy) as usize][(mx + dx) as usize] == STONE {
                grid[(cy + dy) as usize][(cx + dx) as usize] = STONE;
            }
            my = my + dy;
            mx = mx + dx;
            grid[my as usize][mx as usize] = EMPTY;
        }
    }
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (y, x, c)))
        .filter(|&(_, _, c)| c == STONE)
        .map(|(y, x, _)| (y * 100) + x)
        .sum::<usize>() as u32
}

#[aoc(day15, part2, d15p2)]
pub fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input/2024/day15.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 1552463);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(INPUT), 1554058);
    // }
}
