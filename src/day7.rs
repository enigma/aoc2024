use aoc_runner_derive::aoc;

fn solve_dfs(line: &str, part2: bool) -> u64 {
    if let Some((sgoal, sterms)) = line.split_once(':') {
        let goal = sgoal
            .as_bytes()
            .iter()
            .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64);
        let terms: Vec<u64> = sterms
            .split_ascii_whitespace()
            .map(|t| {
                t.as_bytes()
                    .iter()
                    .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64)
            })
            .collect();
        let termslog10 = if part2 {
            terms
                .iter()
                .map(|&t| (t as f64).log10().floor() as u32)
                .map(|log10| 10u64.pow(log10 + 1))
                .collect()
        } else {
            Vec::new()
        };
        let mut fringe = Vec::with_capacity(12);
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
            let term = unsafe { terms.get_unchecked(idx) };
            fringe.push((idx + 1, partial * term));
            fringe.push((idx + 1, partial + term));
            if part2 {
                let mul = unsafe { termslog10.get_unchecked(idx) };
                fringe.push((idx + 1, partial * mul + term));
            }
        }
        return 0;
    }
    0
}

fn recurse(goal: u64, partial: u64, sterms: &str, part2: bool) -> u64 {
    if partial > goal {
        return 0;
    }
    if sterms.is_empty() {
        return if partial == goal { goal } else { 0 };
    }
    let (sterm, rest) = match sterms.split_once(' ') {
        Some((term, rest)) => (term, rest),
        None => (sterms, ""),
    };
    let term = sterm
        .as_bytes()
        .iter()
        .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64);
    let times = recurse(goal, partial * term, rest, part2);
    if times > 0 {
        return times;
    }
    let plus = recurse(goal, partial + term, rest, part2);
    if plus > 0 {
        return plus;
    }
    if part2 {
        let term = sterm
            .as_bytes()
            .iter()
            .fold(partial, |acc, b| acc * 10 + (b - b'0') as u64);
        return recurse(goal, term, rest, part2);
    }
    0
}

fn solve_recurse(line: &str, part2: bool) -> u64 {
    if let Some((sgoal, sterms)) = line.split_once(':') {
        let goal = sgoal
            .as_bytes()
            .iter()
            .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64);
        return recurse(goal, 0, sterms, part2);
    }
    0
}

#[aoc(day7, part1, base)]
pub fn part1(input: &str) -> u64 {
    input.lines().map(|line| solve_dfs(line, false)).sum()
}

#[aoc(day7, part2, base)]
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let s1 = solve_dfs(line, false);
            return if s1 == 0 { solve_dfs(line, true) } else { s1 };
        })
        .sum()
}

#[aoc(day7, part1, recurse)]
pub fn part1_recurse(input: &str) -> u64 {
    input.lines().map(|line| solve_recurse(line, false)).sum()
}

#[aoc(day7, part2, recurse)]
pub fn part2_recurse(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let s1 = solve_recurse(line, false);
            return if s1 == 0 {
                solve_recurse(line, true)
            } else {
                s1
            };
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
        assert_eq!(part1_recurse(INPUT), 538191549061);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 34612812972206);
        assert_eq!(part2_recurse(INPUT), 34612812972206);
    }
}
