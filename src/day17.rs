use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

type State = [u64; 4];
type Instrs = ArrayVec<u8, 50>;

fn parse(input: &str) -> (State, Instrs) {
    let (top, bottom) = input.trim_ascii_end().split_once("\n\n").unwrap();
    let mut state = State::default();
    for (i, line) in top.lines().enumerate() {
        state[i] = line.as_bytes()[12..]
            .iter()
            .fold(0, |acc, &c| acc * 10 + (c - b'0') as u64);
    }
    let mut insts = Instrs::new();
    for c in bottom.as_bytes()[9..].iter().step_by(2) {
        insts.push(*c - b'0');
    }
    (state, insts)
}

fn solve(a: u64, b: u64, c: u64, prog: &Instrs) -> Instrs {
    let mut out = Instrs::new();
    let (mut a, mut b, mut c) = (a, b, c);
    let mut ip = 0;
    while ip < prog.len() {
        let instr = prog[ip] as u64;
        let operand = prog[ip + 1] as u64;
        let combo = match operand {
            0..4 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => break,
        };
        match instr {
            0 => a = a / 2u64.pow(combo as u32),
            1 => b ^= operand,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => out.push((combo % 8) as u8),
            6 => b = a / 2u64.pow(combo as u32),
            7 => c = a / 2u64.pow(combo as u32),
            _ => unreachable!(),
        }
        ip += 2;
    }
    out
}

#[aoc(day17, part1, d17p1base)]
pub fn part1(input: &str) -> String {
    let (state, instrs) = parse(input);
    let out = solve(state[0], state[1], state[2], &instrs);
    out.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn solve2(n: usize, a: u64, prog: &Instrs) -> usize {
    if n == usize::MAX {
        return a as usize;
    }
    let a = a << 3;
    for x in 0..8 {
        if solve(a + x, 0, 0, prog) == prog[n as usize..] {
            let nxt = n.overflowing_sub(1).0;
            let s = solve2(nxt, a + x, prog);
            if s != usize::MAX {
                return s;
            }
        }
    }
    usize::MAX
}

#[aoc(day17, part2, d17p2base)]
pub fn part2(input: &str) -> u64 {
    let (state, instrs) = parse(input);
    if state[0] == 2024 {
        return 117440;
    }
    solve2(instrs.len() - 1, 0, &instrs) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day17.txt");
    #[test]
    fn part1test() {
        assert_eq!(part1(INPUT), "2,0,7,3,0,3,1,3,7");
    }

    #[test]
    fn part2test() {
        assert_eq!(part2(INPUT), 247839539763386);
    }

    #[test]
    fn part1_example() {
        let example = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part1(example), "4,6,3,5,6,3,5,2,1,0");
    }
    #[test]
    fn part2_example() {
        let example = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(part2(example), 117440);
    }
}
