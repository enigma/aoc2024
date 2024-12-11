use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;

fn parse(input: &str) -> ArrayVec<u64, 32> {
    let mut res = ArrayVec::new();
    for line in input.trim_ascii_end().split_ascii_whitespace() {
        res.push(
            line.as_bytes()
                .iter()
                .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64),
        );
    }
    res
}

fn split_no_cache(engraving: u64) -> ArrayVec<u64, 2> {
    let mut res = ArrayVec::new();
    if engraving == 0 {
        res.push(1);
    } else {
        let digits = (engraving as f64).log10().floor() as u64 + 1;
        let div = 10u64.pow((digits / 2) as u32);
        if digits % 2 == 0 {
            res.push(engraving / div);
            res.push(engraving % div);
        } else {
            res.push(engraving * 2024);
        }
    }
    res
}

fn blink(times: u32, engravings: &[u64], cache: &mut FxHashMap<(u64, u32), usize>) -> usize {
    let mut res = 0;
    if times == 0 {
        return engravings.len();
    }
    for &engraving in engravings {
        res += {
            if let Some(res) = cache.get(&(engraving, times)) {
                *res
            } else {
                let split = split_no_cache(engraving);
                let er = blink(times - 1, &split, cache);
                cache.insert((engraving, times), er);
                er
            }
        }
    }
    res
}

#[aoc(day11, part1, d11p1base)]
fn part1base(input: &str) -> u64 {
    let x = parse(input);
    let mut cache = FxHashMap::default();
    blink(25, &x, &mut cache) as u64
}

#[aoc(day11, part2, d11p2base)]
fn part2base(input: &str) -> u64 {
    let x = parse(input);
    let mut cache = FxHashMap::default();
    blink(75, &x, &mut cache) as u64
}

#[aoc(day11, part1, d11p1selected)]
pub fn part1(input: &str) -> u64 {
    part1base(input)
}
#[aoc(day11, part2, d11p2selected)]
pub fn part2(input: &str) -> u64 {
    part2base(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day11.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 233875);
        assert_eq!(part1base(INPUT), 233875);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 277444936413293);
        assert_eq!(part2base(INPUT), 277444936413293);
    }
}
