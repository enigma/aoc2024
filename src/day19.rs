use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;

type Parsed<'a> = (ArrayVec<&'a str, 500>, ArrayVec<&'a str, 500>);

fn parse<'a>(input: &'a str) -> Parsed<'a> {
    let mut patterns = ArrayVec::new();
    let mut designs = ArrayVec::new();
    let (top, bottom) = input.trim_ascii_end().split_once("\n\n").unwrap();
    for part in top.split(", ") {
        patterns.push(part);
    }

    for line in bottom.lines() {
        designs.push(line);
    }

    (patterns, designs)
}

fn count_ways<'a>(
    design: &'a str,
    patterns: &[&'a str],
    cache: &mut FxHashMap<&'a str, u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(&count) = cache.get(design) {
        return count;
    }

    let count = patterns
        .iter()
        .filter(|&&pattern| design.starts_with(pattern))
        .map(|&pattern| count_ways(&design[pattern.len()..], patterns, cache))
        .sum();

    cache.insert(design, count);
    count
}

#[aoc(day19, part1, day19_part1base)]
pub fn part1hashmap(input: &str) -> u64 {
    let (patterns, designs) = parse(input);
    let mut cache = FxHashMap::default();

    designs
        .iter()
        .map(|&design| count_ways(design, &patterns, &mut cache))
        .filter(|&count| count > 0)
        .count() as u64
}

#[aoc(day19, part2, day19_part2base)]
pub fn part2hashmap(input: &str) -> u64 {
    let (patterns, designs) = parse(input);
    let mut cache =
        FxHashMap::with_capacity_and_hasher(12_000, rustc_hash::FxBuildHasher::default());

    designs
        .iter()
        .map(|&design| count_ways(design, &patterns, &mut cache))
        .sum::<u64>()
}

struct Trie {
    children: FxHashMap<u8, Trie>,
    terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: FxHashMap::default(),
            terminal: false,
        }
    }

    fn insert(&mut self, pattern: &[u8]) {
        if pattern.len() == 0 {
            self.terminal = true;
            return;
        }

        let child = self.children.entry(pattern[0]).or_insert_with(Trie::new);
        child.insert(&pattern[1..]);
    }

    // How TF could I make this work?
    // fn prefixes(&self, design: &[u8]) -> impl Iterator<Item = u64> {
    //     if self.terminal {
    //         iter::once(1u64)
    //     } else if design.is_empty() {
    //         iter::once(0u64)
    //     } else if let Some(child) = self.children.get(&design[..1]) {
    //         child.prefixes(&design[1..])
    //     } else {
    //         iter::empty()
    //     }
    // }
}

type ParsedTrie<'a> = (Trie, ArrayVec<&'a [u8], 500>);

fn parse_trie(input: &str) -> ParsedTrie {
    let mut root = Trie::new();
    let (top, bottom) = input.trim_ascii_end().split_once("\n\n").unwrap();
    for part in top.split(", ") {
        root.insert(part.as_bytes());
    }

    let mut designs = ArrayVec::new();
    for line in bottom.lines() {
        designs.push(line.as_bytes());
    }

    (root, designs)
}

fn solve_trie(parsed_trie: ParsedTrie, reduce_fn: impl Fn(u64, u64) -> u64) -> u64 {
    let (root, designs) = parsed_trie;

    let mut dp = [0u64; 104];

    designs
        .iter()
        .map(|&design| {
            dp.fill(0);
            dp[design.len()] = 1;

            for i in (0..design.len()).rev() {
                let mut node = &root;
                let mut j = 0;
                while let Some(child) = node.children.get(&design[i + j]) {
                    node = child;
                    if node.terminal {
                        dp[i] += dp[i + j + 1];
                    }
                    j += 1;
                    if i + j >= design.len() {
                        break;
                    }
                }
            }
            dp[0]
        })
        .reduce(reduce_fn)
        .unwrap()
}

#[aoc(day19, part1, day19_part1trie)]
pub fn part1trie(input: &str) -> u64 {
    solve_trie(parse_trie(input), |a, b| a + if b > 0 { 1 } else { 0 })
}

#[aoc(day19, part2, day19_part2trie)]
pub fn part2trie(input: &str) -> u64 {
    solve_trie(parse_trie(input), |a, b| a + b)
}

pub fn part1(input: &str) -> u64 {
    part2trie(input)
}

pub fn part2(input: &str) -> u64 {
    part2trie(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day19.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1hashmap(INPUT), 236);
        assert_eq!(part1trie(INPUT), 236);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2hashmap(INPUT), 643685981770598);
        assert_eq!(part2trie(INPUT), 643685981770598);
    }
}
