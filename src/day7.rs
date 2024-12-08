use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

fn recurse(goal: u64, partial: u64, terms: &[u64], part2: bool) -> u64 {
    if terms.is_empty() {
        return if partial == 0 { goal } else { 0 };
    }
    if let Some(&term) = terms.last() {
        let rest = &terms[0..terms.len() - 1];
        if partial % term == 0 && recurse(goal, partial / term, rest, part2) > 0 {
            return goal;
        }
        if partial >= term && recurse(goal, partial - term, rest, part2) > 0 {
            return goal;
        }
        if part2 {
            let mul = 10u64.pow(1 + (term as f64).log10().floor() as u32);
            if partial % mul == term && recurse(goal, (partial - term) / mul, rest, part2) > 0 {
                return goal;
            }
        }
        return 0;
    } else {
        if partial == 0 {
            goal
        } else {
            0
        }
    }
}

fn solve_recurse(line: &str, part2: bool) -> u64 {
    if let Some((sgoal, sterms)) = line.split_once(':') {
        let goal = sgoal
            .as_bytes()
            .iter()
            .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64);
        let mut terms = ArrayVec::<u64, 12>::new();
        sterms.split_ascii_whitespace().for_each(|t| {
            terms.push(
                t.as_bytes()
                    .iter()
                    .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64),
            );
        });
        return recurse(goal, goal, &terms, part2);
    }
    0
}

fn solve_dfs_end(line: &str, part2: bool) -> u64 {
    if let Some((sgoal, sterms)) = line.split_once(':') {
        let goal = sgoal
            .as_bytes()
            .iter()
            .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64);
        let mut terms = [0u64; 12];
        let mut termscount = 0;
        sterms
            .split_ascii_whitespace()
            .enumerate()
            .for_each(|(i, t)| unsafe {
                termscount += 1;
                *terms.get_unchecked_mut(i) = t
                    .as_bytes()
                    .iter()
                    .fold(0u64, |acc, b| acc * 10 + (b - b'0') as u64)
            });
        let mut termslog10 = [0u64; 12];
        if part2 {
            (0..termscount)
                .map(|i| (*unsafe { terms.get_unchecked(i) } as f64).log10().floor() as u32)
                .map(|log10| 10u64.pow(log10 + 1))
                .enumerate()
                .for_each(|(i, mul)| unsafe { *termslog10.get_unchecked_mut(i) = mul });
        }
        let mut fringe = ArrayVec::<(usize, u64), 24>::new();
        fringe.push((termscount, goal));
        while let Some((idx, partial)) = fringe.pop() {
            if idx == 0 {
                if partial == 0 {
                    return goal;
                }
                continue;
            }
            let idx = idx - 1;
            let term = *unsafe { terms.get_unchecked(idx) };
            if term <= partial {
                fringe.push((idx, partial - term));
            }
            if partial % term == 0 {
                fringe.push((idx, partial / term));
            }
            if part2 {
                let mul = *unsafe { termslog10.get_unchecked(idx) };
                if partial % mul == term {
                    fringe.push((idx, (partial - term) / mul));
                }
            }
        }
        return 0;
    }
    0
}

#[aoc(day7, part1, base)]
pub fn part1(input: &str) -> u64 {
    input.lines().map(|line| solve_dfs_end(line, false)).sum()
}

#[aoc(day7, part2, base)]
pub fn part2(input: &str) -> u64 {
    input.lines().map(|line| solve_dfs_end(line, true)).sum()
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
