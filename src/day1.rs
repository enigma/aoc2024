const SIZE: usize = 1000;

#[aoc(day1, part1, base)]
pub fn part1(input: &str) -> u32 {
    let mut lhs = [0u32; SIZE];
    let mut rhs = [0u32; SIZE];

    for (i, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        lhs[i] = (0..5).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32);
        rhs[i] = (8..13).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32);
    }

    lhs.sort_unstable();
    rhs.sort_unstable();

    (0..SIZE).map(|i| lhs[i].abs_diff(rhs[i])).sum()
}

#[inline]
fn radix_sort(arr: &mut [u32; SIZE]) {
    let mut temp = [0u32; SIZE];
    let mut counts = [0u32; 256];

    for shift in [0, 8, 16] {
        counts.fill(0);

        let mut i = 0;
        while i + 4 <= SIZE {
            let b1 = ((arr[i] >> shift) & 0xFF) as usize;
            let b2 = ((arr[i + 1] >> shift) & 0xFF) as usize;
            let b3 = ((arr[i + 2] >> shift) & 0xFF) as usize;
            let b4 = ((arr[i + 3] >> shift) & 0xFF) as usize;

            counts[b1] += 1;
            counts[b2] += 1;
            counts[b3] += 1;
            counts[b4] += 1;

            i += 4;
        }

        while i < SIZE {
            counts[((arr[i] >> shift) & 0xFF) as usize] += 1;
            i += 1;
        }

        for i in 1..256 {
            counts[i] += counts[i - 1];
        }

        for &num in arr.iter().rev() {
            let byte = ((num >> shift) & 0xFF) as usize;
            counts[byte] -= 1;
            temp[counts[byte] as usize] = num;
        }

        arr.copy_from_slice(&temp);
    }
}

#[aoc(day1, part1, d1p1radixsort)]
pub fn part1radixsort(input: &str) -> u32 {
    let mut lhs = [0u32; SIZE];
    let mut rhs = [0u32; SIZE];

    for (i, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        lhs[i] = (0..5).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32);
        rhs[i] = (8..13).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32);
    }

    radix_sort(&mut lhs);
    radix_sort(&mut rhs);

    (0..SIZE).map(|i| lhs[i].abs_diff(rhs[i])).sum()
}

#[aoc(day1, part2, base)]
pub fn part2(input: &str) -> u32 {
    let mut lhs = [0u32; SIZE];
    let mut rhs = [0u8; 100_000];

    for (i, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        lhs[i] = (0..5).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32);
        let val = (8..13).fold(0, |acc, j| acc * 10 + (bytes[j] - b'0') as u32) as usize;
        rhs[val] += 1;
    }

    (0..SIZE)
        .map(|i| lhs[i] * (rhs[lhs[i] as usize] as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day1.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1646452);
        assert_eq!(part1radixsort(INPUT), 1646452);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 23609874);
    }
}
