use aoc_runner_derive::aoc;

fn solve(line: &str, part2: bool) -> u64 {
    if let Some((sgoal, sterms)) = line.split_once(':') {
        let goal = sgoal
            .as_bytes()
            .iter()
            .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64);
        let terms: Vec<u64> = sterms
            .split_ascii_whitespace()
            .map(|t| t.parse::<u64>().unwrap())
            .collect();
        let mut fringe = Vec::with_capacity(32);
        fringe.push((1usize, terms[0]));
        while let Some((idx, partial)) = fringe.pop() {
            if partial > goal {
                continue;
            }
            if idx >= terms.len() {
                if partial == goal {
                    return goal;
                }
                continue;
            }
            let term = terms[idx];
            fringe.push((idx + 1, partial * term));
            fringe.push((idx + 1, partial + term));
            if part2 {
                let log10 = (term as f64).log10().floor() as u32;
                let mul = 10u64.pow(log10 + 1);
                fringe.push((idx + 1, partial * mul + term));
            }
        }
        return 0;
    }
    0
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    input.lines().map(|line| solve(line, false)).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let s1 = solve(line, false);
            return if s1 == 0 { solve(line, true) } else { s1 };
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day7.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 538191549061);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 34612812972206);
    }
}
