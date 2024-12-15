use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

type Grid<const H: usize, const W: usize> = ArrayVec<ArrayVec<u8, W>, H>;
type Pos = (usize, usize);
type Dir = (isize, isize);

fn parse1(input: &str) -> (Grid<50, 50>, Pos, Vec<Dir>) {
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

fn parse2(input: &str) -> (Grid<50, 100>, Pos, Vec<Dir>) {
    let mut grid = Grid::new();
    let mut me = (0, 0);
    let (grid_s, dir_s) = input.split_once("\n\n").unwrap();
    for (y, line) in grid_s.lines().enumerate() {
        let mut row = ArrayVec::new();
        for (x, &c) in line.as_bytes().iter().enumerate() {
            row.extend(match c {
                b'@' => {
                    me = (y, x * 2);
                    [b'.', b'.']
                }
                STONE => [STONE_L, STONE_R],
                _ => [c, c],
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

fn can_move(grid: &Grid<50, 100>, cy: isize, cx: isize, dy: isize, dx: isize) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    let (ny, nx) = (cy + dy, cx + dx);
    if ny < 0 || ny >= height as isize || nx < 0 || nx >= width as isize {
        return false;
    }
    let nxt = grid[ny as usize][nx as usize];
    if nxt == EMPTY {
        return true;
    }
    match nxt {
        STONE_L => {
            can_move(grid, ny, nx, dy, dx) && (dy == 0 || can_move(grid, ny, nx + 1, dy, dx))
        }
        STONE_R => {
            can_move(grid, ny, nx, dy, dx) && (dy == 0 || can_move(grid, ny, nx - 1, dy, dx))
        }
        _ => false,
    }
}

fn mover(grid: &mut Grid<50, 100>, cy: isize, cx: isize, dy: isize, dx: isize) {
    let (ny, nx) = (cy + dy, cx + dx);
    let nxt = grid[ny as usize][nx as usize];
    if dy == 0 {
        if nxt == STONE_L || nxt == STONE_R {
            mover(grid, ny, nx, dy, dx);
        }
        grid[ny as usize][nx as usize] = grid[cy as usize][cx as usize];
        grid[cy as usize][cx as usize] = EMPTY;
        return;
    }
    match nxt {
        STONE_L => {
            mover(grid, ny, nx, dy, dx);
            mover(grid, ny, nx + 1, dy, dx);
            {
                let row = &mut grid[(ny + dy) as usize];
                row[nx as usize] = STONE_L;
                row[(nx + 1) as usize] = STONE_R;
            }
            {
                let row = &mut grid[ny as usize];
                row[nx as usize] = EMPTY;
                row[(nx + 1) as usize] = EMPTY;
            }
            grid[cy as usize][cx as usize] = EMPTY;
        }
        STONE_R => {
            mover(grid, ny, nx, dy, dx);
            mover(grid, ny, nx - 1, dy, dx);
            {
                let row = &mut grid[(ny + dy) as usize];
                row[(nx - 1) as usize] = STONE_L;
                row[nx as usize] = STONE_R;
            }
            {
                let row = &mut grid[ny as usize];
                row[(nx - 1) as usize] = EMPTY;
                row[nx as usize] = EMPTY;
            }
            grid[cy as usize][cx as usize] = EMPTY;
        }
        _ => {}
    }
}

#[aoc(day15, part2, d15p2)]
pub fn part2(input: &str) -> u32 {
    let (mut grid, me, dirs) = parse2(input);
    let height = grid.len();
    let width = grid[0].len();
    let (mut my, mut mx) = (me.0 as isize, me.1 as isize);
    for (dy, dx) in dirs {
        let (cy, cx) = (my, mx);
        if can_move(&grid, cy, cx, dy, dx) {
            mover(&mut grid, cy, cx, dy, dx);
            my = cy + dy;
            mx = cx + dx;
        }
    }
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| (y, x, c)))
        .filter(|&(_, _, c)| c == STONE_L)
        .map(|(y, x, _)| (y * 100) + x)
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input/2024/day15.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 1552463);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1554058);
    }
}
