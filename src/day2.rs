use aoc_runner_derive::aoc;

fn is_safe(line: &str, skip: Option<usize>) -> bool {
    let (mut prev, mut prevprev) = (0, 0i32);
    let mut valid = true;
    let mut bad = 0;
    for (i, sn) in line.split_whitespace().enumerate() {
        if Some(i) == skip {
            continue;
        }
        let cur = sn
            .as_bytes()
            .iter()
            .fold(0, |acc, b| acc * 10 + (b - b'0') as i32);
        if prev == 0 && prevprev == 0 {
            prev = cur;
            continue;
        }
        let delta = cur - prev;
        if prevprev != 0 && delta.signum() != (prev - prevprev).signum() {
            valid = false;
            bad = i;
            break;
        }
        let delta_abs = delta.abs();
        if delta_abs < 1 || delta_abs > 3 {
            valid = false;
            bad = i;
            break;
        }
        prevprev = prev;
        prev = cur;
    }
    if valid {
        return true;
    }
    if skip.is_none() {
        for i in (bad.saturating_sub(2))..(bad + 1) {
            if is_safe(line, Some(i)) {
                return true;
            }
        }
    }
    false
}

#[aoc(day2, part1, base)]
fn part1(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        if is_safe(line, Some(usize::MAX)) {
            score += 1;
        }
    }
    score as u32
}

#[aoc(day2, part2, base)]
fn part2(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        if is_safe(line, None) {
            score += 1;
        }
    }
    score as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 257);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 328);
    }
}
