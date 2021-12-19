fn execute(cur: &mut Vec<usize>) -> usize {
    let mut pos = 0;
    let len = cur.len();
    while pos < len {
        match cur[pos] {
            1 => {
                let a = cur[pos + 1];
                let b = cur[pos + 2];
                let r = cur[pos + 3];
                cur[r] = cur[a] + cur[b];
                pos += 4;
            }
            2 => {
                let a = cur[pos + 1];
                let b = cur[pos + 2];
                let r = cur[pos + 3];
                cur[r] = cur[a] * cur[b];
                pos += 4;
            }
            99 => {
                break;
            }
            _ => {
                panic!("UNKNOWN OP {}", cur[pos]);
            }
        }
    }
    return cur[0];
}

fn part1(input: &str) -> usize {
    let mut program = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    program[1] = 12;
    program[2] = 2;
    let result = execute(&mut program);
    return result;
}

fn part2(input: &str) -> usize {
    let program = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut cur = program.clone();
            cur[1] = noun;
            cur[2] = verb;
            if execute(&mut cur) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}
