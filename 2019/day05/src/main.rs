fn is_immediate_mode(instruction: isize, n: u32) -> bool {
    ((instruction) / (10 * (10_isize).pow(n))) % 10 == 1
}

fn read_param(v: &Vec<isize>, pos: usize, n: u32, instruction: isize) -> isize {
    let idx: usize = pos + n as usize;
    if is_immediate_mode(instruction, n) {
        v[idx]
    } else {
        v[v[idx] as usize]
    }
}

const DEBUG: bool = false;

fn execute(cur: &mut Vec<isize>, input: isize) -> isize {
    let mut pos = 0;
    let len = cur.len();
    let mut last_output = -1;
    while pos < len {
        let instruction = cur[pos];
        match instruction % 100 {
            1 => {
                let a = read_param(&cur, pos, 1, instruction);
                let b = read_param(&cur, pos, 2, instruction);
                let r = cur[pos + 3];
                if DEBUG {
                    println!("ADD {} + {} -> cur[{}] = {}", a, b, r, a + b);
                }
                cur[r as usize] = a + b;
                pos += 4;
            }
            2 => {
                let a = read_param(&cur, pos, 1, instruction);
                let b = read_param(&cur, pos, 2, instruction);
                let r = cur[pos + 3];
                if DEBUG {
                    println!("MUL {} * {} -> cur[{}] = {}", a, b, r, a * b);
                }
                cur[r as usize] = a * b;
                pos += 4;
            }
            3 => {
                let v = cur[pos + 1] as usize;
                if DEBUG {
                    println!("INPUT cur[{}] = {}", v, input);
                }
                cur[v as usize] = input;
                pos += 2;
            }
            4 => {
                let v = read_param(&cur, pos, 1, instruction);
                if DEBUG {
                    println!("OUTPUT {}", v);
                }
                last_output = v;
                pos += 2;
            }
            5 => {
                let a = read_param(&cur, pos, 1, instruction);
                if DEBUG {
                    println!("JMPIF {} > 0", a);
                }
                if a > 0 {
                    let b = read_param(&cur, pos, 2, instruction);
                    pos = b as usize;
                    if DEBUG {
                        dbg!(pos);
                    }
                } else {
                    pos += 3;
                }
            }
            6 => {
                let a = read_param(&cur, pos, 1, instruction);
                if DEBUG {
                    println!("JMPIF {} == 0", a);
                }
                if a == 0 {
                    let b = read_param(&cur, pos, 2, instruction);
                    pos = b as usize;
                    if DEBUG {
                        dbg!(pos);
                    }
                } else {
                    pos += 3;
                }
            }
            7 => {
                let a = read_param(&cur, pos, 1, instruction);
                let b = read_param(&cur, pos, 2, instruction);
                let c = cur[pos + 3];
                if DEBUG {
                    println!(
                        "LT {} < {} -> cur[{}] = {}",
                        a,
                        b,
                        c,
                        if a < b { 1 } else { 0 }
                    );
                }
                cur[c as usize] = if a < b { 1 } else { 0 };
                pos += 4;
            }
            8 => {
                let a = read_param(&cur, pos, 1, instruction);
                let b = read_param(&cur, pos, 2, instruction);
                let c = cur[pos + 3];
                if DEBUG {
                    println!(
                        "EQ {} == {} -> cur[{}] = {}",
                        a,
                        b,
                        c,
                        if a == b { 1 } else { 0 }
                    );
                }
                cur[c as usize] = if a == b { 1 } else { 0 };
                pos += 4;
            }
            99 => {
                if DEBUG {
                    println!("TERMINATE");
                }
                break;
            }
            _ => {
                panic!("UNKNOWN OP {} at {}", cur[pos], pos);
            }
        }
    }
    last_output
}

fn part1(input: &str) -> isize {
    let mut program = input
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    execute(&mut program, 1)
}

fn part2(prog: &str, input: isize) -> isize {
    let mut program = prog
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    execute(&mut program, input)
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 1: {}", part2(include_str!("in.txt"), 5));
}

#[test]
fn test_immediate_mode_flag() {
    assert_eq!(is_immediate_mode(1002, 1), false);
    assert_eq!(is_immediate_mode(1002, 2), true);
    assert_eq!(is_immediate_mode(1002, 3), false);
    assert_eq!(is_immediate_mode(11002, 3), true);
    assert_eq!(is_immediate_mode(11002, 2), true);
    assert_eq!(is_immediate_mode(11002, 1), false);
    assert_eq!(is_immediate_mode(102, 1), true);
    assert_eq!(is_immediate_mode(102, 2), false);
    assert_eq!(is_immediate_mode(102, 3), false);
}

#[test]
fn test_part1() {
    assert_eq!(part1("1002,4,3,4,33"), -1);
}

#[test]
fn test_part2() {
    assert_eq!(part2("3,9,8,9,10,9,4,9,99,-1,8", 8), 1);
    assert_eq!(part2("3,9,8,9,10,9,4,9,99,-1,8", 6), 0);
    assert_eq!(
        part2("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 8),
        1000
    );
    assert_eq!(
        part2("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 7),
        999
    );
}
