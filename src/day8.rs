use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
    let mut antennas: [ArrayVec<(usize, usize), 5>; 128] = std::array::from_fn(|_| ArrayVec::new());
    input.lines().enumerate().for_each(|(y, line)| {
        line.as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b != b'.' {
                unsafe {
                    antennas.get_unchecked_mut(b as usize).push((x, y));
                }
            }
        })
    });
    let mut count = 0;
    let mut seen = [[false; 50]; 50];
    antennas
        .iter()
        .filter(|antenna| antenna.len() > 0)
        .for_each(|ants| {
            for (i, &(uay, uax)) in ants[0..ants.len() - 1].iter().enumerate() {
                ants[i + 1..].iter().for_each(|&(uby, ubx)| {
                    let dy = uay as isize - uby as isize;
                    let dx = uax as isize - ubx as isize;
                    for f in [1, -2] {
                        let ny = (uay as isize) + f * dy;
                        let nx = (uax as isize) + f * dx;
                        if 0 <= ny && ny < 50 && 0 <= nx && nx < 50 {
                            let uny = ny as usize;
                            let unx = nx as usize;
                            if !seen[uny][unx] {
                                count += 1;
                                seen[uny][unx] = true;
                            }
                        }
                    }
                })
            }
        });
    count
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    let mut antennas: [ArrayVec<(usize, usize), 5>; 128] = std::array::from_fn(|_| ArrayVec::new());
    input.lines().enumerate().for_each(|(y, line)| {
        line.as_bytes().iter().enumerate().for_each(|(x, &b)| {
            if b != b'.' {
                unsafe {
                    antennas.get_unchecked_mut(b as usize).push((x, y));
                }
            }
        })
    });
    let mut count = 0;
    let mut seen = [[false; 50]; 50];
    antennas
        .iter()
        .filter(|antenna| antenna.len() > 0)
        .for_each(|ants| {
            for (i, &(uay, uax)) in ants[0..ants.len() - 1].iter().enumerate() {
                ants[i + 1..].iter().for_each(|&(uby, ubx)| {
                    let dy = uay as isize - uby as isize;
                    let dx = uax as isize - ubx as isize;
                    for sign in [1, -1] {
                        let mut cnt = 0;
                        loop {
                            let ny = (uay as isize) + cnt * sign * dy;
                            let nx = (uax as isize) + cnt * sign * dx;
                            if 0 <= ny && ny < 50 && 0 <= nx && nx < 50 {
                                let uny = ny as usize;
                                let unx = nx as usize;
                                if !seen[uny][unx] {
                                    count += 1;
                                    seen[uny][unx] = true;
                                }
                            } else {
                                break;
                            }
                            cnt += 1;
                        }
                    }
                })
            }
        });
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day8.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 394);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1277);
    }
}
