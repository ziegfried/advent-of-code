use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

use Instruction::*;

fn parse_line(s: &str) -> Instruction {
    let parts = s.splitn(2, " ").collect::<Vec<&str>>();
    let n = (&parts[1]).parse::<isize>().unwrap();
    match parts[0] {
        "acc" => Acc(n),
        "jmp" => Jmp(n),
        "nop" => Nop(n),
        _ => panic!("invalid op"),
    }
}

fn exec(ops: Vec<Instruction>) -> (isize, bool) {
    let mut accum: isize = 0;
    let mut seen = HashSet::new();
    let mut i: usize = 0;
    loop {
        if seen.contains(&i) {
            // infinite loop detected
            break;
        }
        seen.insert(i);

        let inst = ops.get(i);
        match inst {
            Some(Nop(_)) => {
                i += 1;
            }
            Some(Acc(n)) => {
                accum += n;
                i += 1;
            }
            Some(Jmp(n)) => {
                let t: isize = i as isize + n;
                i = t.clone() as usize;
            }
            _ => {
                if i == ops.len() {
                    return (accum, true);
                }
                panic!("invalid instruction pointer");
            }
        };
    }

    return (accum, false);
}

fn main() {
    let instructions = include_str!("../in.txt")
        .split('\n')
        .map(|s| parse_line(s))
        .collect::<Vec<Instruction>>();

    let (result, _) = exec(instructions.clone());
    println!("Part 1: {}", result);

    for i in 0..(instructions.len()) {
        match instructions.get(i) {
            Some(Jmp(n)) => {
                let mut cloned = instructions.clone();
                cloned[i] = Nop(*n);
                let (accum, completed) = exec(cloned);
                if completed {
                    println!("Part 2: {}", accum);
                    return;
                }
            }
            Some(Nop(n)) => {
                let mut cloned = instructions.clone();
                cloned[i] = Jmp(*n);
                let (accum, completed) = exec(cloned);
                if completed {
                    println!("Part 2: {}", accum);
                    return;
                }
            }
            _ => {
                // ignore
            }
        }
    }
}
