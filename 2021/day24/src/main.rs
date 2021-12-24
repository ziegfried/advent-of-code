use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
enum Value {
    Var(Variable),
    Literal(i64),
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Value),
    Mul(Variable, Value),
    Div(Variable, Value),
    Mod(Variable, Value),
    Eql(Variable, Value),
}

use Instruction::*;
use Variable::*;

fn parse_variable(s: &str) -> Variable {
    match s {
        "w" => W,
        "x" => X,
        "y" => Y,
        "z" => Z,
        v => panic!("invalid variable {}", v),
    }
}

fn parse_value(s: &str) -> Value {
    use Value::*;
    match s {
        "w" => Var(W),
        "x" => Var(X),
        "y" => Var(Y),
        "z" => Var(Z),
        v => Literal(v.parse::<i64>().unwrap()),
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            match parts[0] {
                "inp" => Inp(parse_variable(parts[1])),
                "add" => Add(parse_variable(parts[1]), parse_value(parts[2])),
                "mul" => Mul(parse_variable(parts[1]), parse_value(parts[2])),
                "div" => Div(parse_variable(parts[1]), parse_value(parts[2])),
                "mod" => Mod(parse_variable(parts[1]), parse_value(parts[2])),
                "eql" => Eql(parse_variable(parts[1]), parse_value(parts[2])),
                _ => panic!(),
            }
        })
        .collect()
}

#[inline(always)]
fn read_var(var: Variable, variables: &[i64; 4]) -> i64 {
    variables[match var {
        W => 0,
        X => 1,
        Y => 2,
        Z => 3,
    }]
}

#[inline(always)]
fn write_var(var: Variable, val: i64, variables: &mut [i64; 4]) {
    variables[match var {
        W => 0,
        X => 1,
        Y => 2,
        Z => 3,
    }] = val;
}

#[inline(always)]
fn resolve_value(val: Value, variables: &[i64; 4]) -> i64 {
    match val {
        Value::Literal(v) => v,
        Value::Var(v) => read_var(v, variables),
    }
}

fn execute(instructions: &[Instruction], input: &[i64], variables: &mut [i64; 4]) -> i64 {
    let mut input = input.iter();
    for i in instructions {
        match i.clone() {
            Inp(v) => {
                write_var(v, *input.next().unwrap(), variables);
            }
            Add(a, b) => {
                let res = read_var(a, variables) + resolve_value(b, variables);
                write_var(a, res, variables);
            }
            Mul(a, b) => {
                let res = read_var(a, variables) * resolve_value(b, variables);
                write_var(a, res, variables);
            }
            Div(a, b) => {
                let res = read_var(a, variables) / resolve_value(b, variables);
                write_var(a, res, variables);
            }
            Mod(a, b) => {
                let res = read_var(a, variables) % resolve_value(b, variables);
                write_var(a, res, variables);
            }
            Eql(a, b) => {
                let av = read_var(a, variables);
                let bv = resolve_value(b, variables);
                write_var(a, if av == bv { 1 } else { 0 }, variables);
            }
        }
    }
    read_var(Z, variables)
}

fn split_into_chunks(instructions: &Vec<Instruction>) -> Vec<Vec<Instruction>> {
    let mut result = vec![];
    let mut idx = 0;
    loop {
        if let Some((next, _)) = instructions
            .iter()
            .enumerate()
            .find(|(i, inst)| i > &idx && matches!(inst, Inp(_)))
        {
            result.push(
                instructions[idx..next]
                    .into_iter()
                    .map(|i| i.clone())
                    .collect(),
            );
            idx = next;
        } else {
            result.push(instructions[idx..].into_iter().map(|i| i.clone()).collect());
            break;
        }
    }
    result
}

fn find_input(
    chunk: usize,
    vars: [i64; 4],
    chunks: &Vec<Vec<Instruction>>,
    dead_ends: &mut HashSet<(usize, [i64; 4])>,
    value: i64,
    max: bool,
) -> Option<i64> {
    if dead_ends.contains(&(chunk, vars)) {
        return None;
    }
    let instructions = &chunks[chunk];
    let digits: Vec<i64> = if max {
        (1..=9).rev().collect()
    } else {
        (1..=9).collect()
    };
    for d in digits {
        let value = value * 10 + d;
        let mut new_vars = vars.clone();
        let z = execute(instructions, &vec![d], &mut new_vars);
        if chunk == chunks.len() - 1 {
            if z == 0 {
                return Some(value);
            }
        } else if let Some(res) = find_input(chunk + 1, new_vars, chunks, dead_ends, value, max) {
            return Some(res);
        }
    }
    dead_ends.insert((chunk, vars));
    None
}

fn part1(input: &str) -> i64 {
    let instructions = parse(input);
    let chunks = split_into_chunks(&instructions);
    find_input(0, Default::default(), &chunks, &mut HashSet::new(), 0, true).unwrap()
}

fn part2(input: &str) -> i64 {
    let instructions = parse(input);
    let chunks = split_into_chunks(&instructions);
    find_input(
        0,
        Default::default(),
        &chunks,
        &mut HashSet::new(),
        0,
        false,
    )
    .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}
