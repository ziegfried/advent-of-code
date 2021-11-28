mod intcode;
use intcode::{parse_program, IntcodeComputer};
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    let program = parse_program(input);
    let mut computer = IntcodeComputer::create(program, vec![]);
    let mut block_tiles = HashSet::<(i64, i64)>::new();

    loop {
        let x = match computer.run_until_next_output() {
            Some(v) => v,
            None => break,
        };
        let y = computer.run_until_next_output().unwrap();
        let tile = computer.run_until_next_output().unwrap();

        if tile == 2 {
            block_tiles.insert((x, y));
        } else {
            block_tiles.remove(&(x, y));
        }
    }

    block_tiles.len()
}

fn part2(input: &str) -> i64 {
    let mut program = parse_program(input);
    program[0] = 2;
    let mut computer = IntcodeComputer::create(program, vec![]);
    computer.set_fallback_input(0);
    let mut score: i64 = 0;
    let mut ball: i64 = 0;
    let mut paddle: i64 = 0;

    loop {
        let x = match computer.run_until_next_output() {
            Some(v) => v,
            None => break,
        };
        let y = computer.run_until_next_output().unwrap();
        let tile = computer.run_until_next_output().unwrap();

        if x == -1 && y == 0 {
            score = tile;
        } else {
            if tile == 3 {
                paddle = x;
            }
            if tile == 4 {
                ball = x;
            }
            if paddle > ball {
                computer.set_fallback_input(-1);
            } else if paddle < ball {
                computer.set_fallback_input(1)
            } else {
                computer.set_fallback_input(0)
            }
        }
    }
    score
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}
