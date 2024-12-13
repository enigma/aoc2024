use aoc_runner_derive::aoc;

type Int = i64;
type Coord = (Int, Int);

#[inline(always)]
fn parse_int(line: &[u8]) -> Int {
    line[2..]
        .iter()
        .fold(0, |acc, b| acc * 10 + (b - b'0') as Int)
}

#[inline(always)]
fn parse_button(line: &str, skipping: usize) -> Coord {
    let mut xx = line
        .split_ascii_whitespace()
        .skip(skipping)
        .map(|s| s.as_bytes());
    let x = xx.next().unwrap();
    let y = xx.next().unwrap();
    (parse_int(&x[..x.len() - 1]), parse_int(y))
}

#[inline(always)]
fn parse_button2(line: &[u8]) -> Coord {
    let lhs_msd = unsafe { line.get_unchecked(12) } - b'0';
    let lhs_lsd = unsafe { line.get_unchecked(13) } - b'0';
    let rhs_msd = unsafe { line.get_unchecked(18) } - b'0';
    let rhs_lsd = unsafe { line.get_unchecked(19) } - b'0';
    (
        (lhs_msd * 10 + lhs_lsd) as Int,
        (rhs_msd * 10 + rhs_lsd) as Int,
    )
}

#[inline(always)]
fn solve_one(a: Coord, b: Coord, c: Coord) -> Int {
    let b_num = a.0 * c.1 - a.1 * c.0;
    let b_den = a.0 * b.1 - a.1 * b.0;
    let bb = b_num / b_den;
    if b_num % b_den != 0 || bb < 0 {
        return 0;
    }
    let a_num = c.0 - bb * b.0;
    let aa = a_num / a.0;
    if a_num % a.0 != 0 || aa < 0 {
        return 0;
    }
    aa * 3 + bb
}

fn solve(input: &str, extra: Int) -> Int {
    let mut total = 0;
    for block in input.trim_ascii_end().split("\n\n") {
        let mut lines = block.lines();
        let a_line = lines.next().unwrap();
        let b_line = lines.next().unwrap();
        let c_line = lines.next().unwrap();

        let a = parse_button2(a_line.as_bytes());
        let b = parse_button2(b_line.as_bytes());
        let mut c = parse_button(c_line, 1);
        c.0 += extra;
        c.1 += extra;
        total += solve_one(a, b, c)
    }
    total
}

#[aoc(day13, part1, d13p1)]
fn part1(input: &str) -> Int {
    solve(input, 0)
}

#[aoc(day13, part2, d13p2)]
fn part2(input: &str) -> Int {
    solve(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day13.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 31552);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 95273925552482);
    }
}
