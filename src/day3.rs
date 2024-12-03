use aoc_runner_derive::aoc;
use regex::Regex;

fn solve_regex(input: &str, part2: bool) -> u32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut doing = true;
    for cap in re.captures_iter(input) {
        match cap.get(0).unwrap().as_str() {
            "do()" => doing = true,
            "don't()" => doing = false,
            _ => {
                if !part2 || doing {
                    let num1 = cap[1]
                        .as_bytes()
                        .iter()
                        .fold(0u32, |acc, b| acc * 10 + (b - b'0') as u32);
                    let num2 = cap[2]
                        .as_bytes()
                        .iter()
                        .fold(0u32, |acc, b| acc * 10 + (b - b'0') as u32);
                    sum += num1 * num2;
                }
            }
        }
    }
    sum
}

fn solve(input: &str, part2: bool) -> u32 {
    let mut sum = 0;
    let mut doing = true;
    let mut i = 0;
    let bytes = input.as_bytes();

    while i < bytes.len() {
        match bytes[i] {
            b'd' => {
                if i + 3 < bytes.len() && &bytes[i..i + 4] == b"do()" {
                    doing = true;
                    i += 3;
                } else if i + 7 < bytes.len() && &bytes[i..i + 7] == b"don't()" {
                    doing = false;
                    i += 7;
                } else {
                    i += 1;
                }
            }
            b'm' => {
                if i + 3 < bytes.len() && &bytes[i..i + 4] == b"mul(" {
                    i += 4;
                    let mut num1 = 0u32;
                    while i < bytes.len() && bytes[i].is_ascii_digit() {
                        num1 = num1 * 10 + (bytes[i] - b'0') as u32;
                        i += 1;
                    }
                    if i >= bytes.len() || bytes[i] != b',' {
                        continue;
                    }
                    i += 1; // skip comma
                    let mut num2 = 0u32;
                    while i < bytes.len() && bytes[i].is_ascii_digit() {
                        num2 = num2 * 10 + (bytes[i] - b'0') as u32;
                        i += 1;
                    }
                    if i >= bytes.len() || bytes[i] != b')' {
                        continue;
                    }
                    i += 1; // skip )
                    if !part2 || doing {
                        sum += num1 * num2;
                    }
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    sum
}

#[aoc(day3, part1, base)]
fn part1(input: &str) -> u32 {
    solve(input, false)
}

#[aoc(day3, part2, base)]
fn part2(input: &str) -> u32 {
    solve(input, true)
}

#[aoc(day3, part1, regex)]
fn part1_regex(input: &str) -> u32 {
    solve_regex(input, false)
}

#[aoc(day3, part2, regex)]
fn part2_regex(input: &str) -> u32 {
    solve_regex(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day3.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 185797128);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 89798695);
    }
}
