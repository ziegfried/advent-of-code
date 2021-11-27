use itertools::Itertools;
use std::cell::RefCell;

fn is_immediate_mode(instruction: isize, param_n: u32) -> bool {
    ((instruction) / (10 * (10_isize).pow(param_n))) % 10 == 1
}

fn read_param(v: &Vec<isize>, pos: usize, n: u32, instruction: isize) -> isize {
    let idx: usize = pos + n as usize;
    if is_immediate_mode(instruction, n) {
        v[idx]
    } else {
        v[v[idx] as usize]
    }
}

#[derive(Debug)]
enum InstructionEffect {
    Output(isize),
    Halt,
    Nothing,
}

use InstructionEffect::*;

fn execute_instruction(
    program: &mut Vec<isize>,
    pos: usize,
    inputs: &mut Vec<isize>,
) -> (InstructionEffect, usize) {
    let instruction = program[pos];
    let mut pos = pos;
    match instruction % 100 {
        1 => {
            let a = read_param(&program, pos, 1, instruction);
            let b = read_param(&program, pos, 2, instruction);
            let r = program[pos + 3];
            println!("ADD({}, {}) -> mem[{}] = {}", a, b, r, a + b);
            program[r as usize] = a + b;
            pos += 4;
        }
        2 => {
            let a = read_param(&program, pos, 1, instruction);
            let b = read_param(&program, pos, 2, instruction);
            let r = program[pos + 3];
            println!("MUL({}, {}) -> mem[{}] = {}", a, b, r, a * b);
            program[r as usize] = a * b;
            pos += 4;
        }
        3 => {
            let v = program[pos + 1] as usize;
            println!("{:?}", inputs);
            if inputs.len() == 0 {
                panic!("END OF INPUT");
            }
            let input = inputs.remove(0);
            println!("INPUT mem[{}] = {}", v, input);
            program[v as usize] = input;
            pos += 2;
        }
        4 => {
            let v = read_param(&program, pos, 1, instruction);
            pos += 2;
            println!("OUTPUT: {}", v);
            return (Output(v), pos);
        }
        5 => {
            let a = read_param(&program, pos, 1, instruction);
            if a > 0 {
                let b = read_param(&program, pos, 2, instruction);
                println!("JUMP: {}", b);
                pos = b as usize;
            } else {
                println!("NO JUMP");
                pos += 3;
            }
        }
        6 => {
            let a = read_param(&program, pos, 1, instruction);
            if a == 0 {
                let b = read_param(&program, pos, 2, instruction);
                println!("JUMP: {}", b);
                pos = b as usize;
            } else {
                println!("NO JUMP");
                pos += 3;
            }
        }
        7 => {
            let a = read_param(&program, pos, 1, instruction);
            let b = read_param(&program, pos, 2, instruction);
            let c = program[pos + 3];
            program[c as usize] = if a < b { 1 } else { 0 };
            println!("GT mem[{}] = {}", c, if a < b { 1 } else { 0 });
            pos += 4;
        }
        8 => {
            let a = read_param(&program, pos, 1, instruction);
            let b = read_param(&program, pos, 2, instruction);
            let c = program[pos + 3];
            program[c as usize] = if a == b { 1 } else { 0 };
            println!("EQ mem[{}] = {}", c, if a == b { 1 } else { 0 });
            pos += 4;
        }
        99 => {
            return (Halt, pos + 1);
        }
        _ => {
            panic!("UNKNOWN OP {} at {}", program[pos], pos);
        }
    }
    return (Nothing, pos);
}

fn execute_program(program: &mut Vec<isize>, inputs: &mut Vec<isize>) -> isize {
    println!("EXECUTE {:?} with INPUT {:?}", program, inputs);
    let mut pos = 0;
    let len = program.len();
    let mut last_output = -1;
    while pos < len {
        let (effect, next_pos) = execute_instruction(program, pos, inputs);
        pos = next_pos;
        match effect {
            Halt => break,
            Output(value) => {
                last_output = value;
            }
            Nothing => {}
        }
    }
    println!("-----\n");
    last_output
}

fn determine_amplifier_output(program: &Vec<isize>, phase_settings: &Vec<u32>) -> isize {
    let mut cur_output: isize = 0;
    for setting in phase_settings {
        let mut program = program.clone();
        let mut inputs = vec![*setting as isize, cur_output];
        cur_output = execute_program(&mut program, &mut inputs);
    }
    cur_output
}

fn part1(input: &str) -> isize {
    let program = input
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    (0..5)
        .permutations(5)
        .map(|settings| determine_amplifier_output(&program, &settings))
        .max()
        .unwrap()
}

struct ProgramInstance {
    program: RefCell<Vec<isize>>,
    inputs: RefCell<Vec<isize>>,
    pos: usize,
    pub halted: bool,
    pub last_output: Option<isize>,
}

impl ProgramInstance {
    pub fn create(program: Vec<isize>, inputs: Vec<isize>) -> ProgramInstance {
        ProgramInstance {
            program: RefCell::new(program.clone()),
            inputs: RefCell::new(inputs.clone()),
            pos: 0,
            halted: false,
            last_output: None,
        }
    }
    fn execute_next(&mut self, input: isize) -> InstructionEffect {
        if self.halted {
            panic!("Program already halted");
        }
        self.inputs.borrow_mut().push(input);
        loop {
            let (effect, next_pos) = execute_instruction(
                &mut self.program.borrow_mut(),
                self.pos,
                &mut self.inputs.borrow_mut(),
            );
            self.pos = next_pos;
            match effect {
                Halt => {
                    self.halted = true;
                    return Halt;
                }
                Output(v) => {
                    self.last_output = Some(v);
                    return Output(v);
                }
                Nothing => {}
            }
        }
    }
}

fn determine_amp_feedback_loop_output(program: &Vec<isize>, phase_settings: &Vec<u32>) -> isize {
    let mut progs = phase_settings
        .iter()
        .map(|setting| ProgramInstance::create(program.clone(), vec![*setting as isize]))
        .collect::<Vec<_>>();
    let mut last_output: isize = 0;
    let mut i = 0;
    while !progs[progs.len() - 1].halted {
        match progs.get_mut(i).unwrap().execute_next(last_output) {
            InstructionEffect::Output(output) => {
                last_output = output;
            }
            _ => {}
        }
        i = (i + 1) % progs.len();
    }
    progs[progs.len() - 1].last_output.unwrap()
}

fn part2(input: &str) -> isize {
    let program = input
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    (5..=9)
        .permutations(5)
        .map(|settings| determine_amp_feedback_loop_output(&program, &settings))
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 1: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(
        part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
        43210
    );
    assert_eq!(
        part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
        54321
    );
}

#[test]
fn test_part2() {
    let mut program =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            .split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
    let res = execute_program(&mut program, &mut vec![9, 8, 7, 6, 5, 0]);
    assert_eq!(res, 5);
}
