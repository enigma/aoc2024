use std::collections::hash_map::Entry;

use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;

fn parse(input: &str) -> Vec<u64> {
    let mut res = Vec::with_capacity(1 << 12);
    for line in input.trim_ascii_end().lines() {
        res.push(
            line.as_bytes()
                .iter()
                .fold(0, |acc, &b| acc * 10 + (b - b'0') as u64),
        );
    }
    res
}

fn next_secret(secret: u64) -> u64 {
    let mut secret = (secret ^ (secret * 64)) % 16777216;
    secret = (secret ^ (secret / 32)) % 16777216;
    (secret ^ (secret * 2048)) % 16777216
}

#[aoc(day22, part1, p1base)]
pub fn part1(input: &str) -> u64 {
    let mut secrets = parse(input);
    for _ in 0..2000 {
        secrets
            .iter_mut()
            .for_each(|secret| *secret = next_secret(*secret));
    }
    secrets.iter().sum()
}

#[aoc(day22, part2, p2base)]
pub fn part2(input: &str) -> u64 {
    let mut secrets = parse(input);
    let mut counter: FxHashMap<_, _> = FxHashMap::default();
    let mut result = 0;
    let mut caches: Vec<FxHashMap<u64, u64>> =
        vec![FxHashMap::with_capacity_and_hasher(1 << 10, Default::default()); secrets.len()];
    let mut hashes = vec![0; secrets.len()];
    let mask = (1 << 20) - 1;

    for i in 0..2000 {
        secrets
            .iter_mut()
            .zip(caches.iter_mut())
            .zip(hashes.iter_mut())
            .for_each(|((secret, cache), hash)| {
                let was = *secret;
                *secret = next_secret(*secret);
                let delta = 10 + (*secret % 10) - (was % 10);
                *hash = ((*hash << 5) + delta) & mask;
                if i >= 4 {
                    if let Entry::Vacant(ee) = cache.entry(*hash) {
                        let s = *secret % 10;
                        ee.insert(s);
                        let e = counter.entry(*hash).or_insert(0);
                        *e += s;
                        result = result.max(*e);
                    }
                }
            });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day22.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 18525593556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 2089);
    }
}
