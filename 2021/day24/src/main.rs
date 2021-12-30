use hashbrown::HashSet;

#[derive(Debug, Clone, Copy)]
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

fn parse(input: &str) -> Vec<Instruction> {
    fn parse_variable(s: &str) -> Result<Variable, ()> {
        match s {
            "w" => Ok(W),
            "x" => Ok(X),
            "y" => Ok(Y),
            "z" => Ok(Z),
            _ => Err(()),
        }
    }
    fn parse_value(s: &str) -> Value {
        use Value::*;
        match parse_variable(s) {
            Ok(var) => Var(var),
            Err(_) => Literal(s.parse::<i64>().unwrap()),
        }
    }
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            match parts[0] {
                "inp" => Inp(parse_variable(parts[1]).unwrap()),
                "add" => Add(parse_variable(parts[1]).unwrap(), parse_value(parts[2])),
                "mul" => Mul(parse_variable(parts[1]).unwrap(), parse_value(parts[2])),
                "div" => Div(parse_variable(parts[1]).unwrap(), parse_value(parts[2])),
                "mod" => Mod(parse_variable(parts[1]).unwrap(), parse_value(parts[2])),
                "eql" => Eql(parse_variable(parts[1]).unwrap(), parse_value(parts[2])),
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

fn execute(instructions: &[Instruction], input: i64, variables: &mut [i64; 4]) -> i64 {
    let mut input = Some(input);
    for i in instructions {
        match i.clone() {
            Inp(v) => {
                write_var(v, input.unwrap(), variables);
                input = None;
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
    let mut input_idx = 0;
    loop {
        assert!(matches!(instructions[input_idx], Inp(_)));
        if let Some((next_input, _)) = instructions
            .iter()
            .enumerate()
            .find(|(i, inst)| i > &input_idx && matches!(inst, Inp(_)))
        {
            result.push(
                instructions[input_idx..next_input]
                    .into_iter()
                    .cloned()
                    .collect(),
            );
            input_idx = next_input;
        } else {
            result.push(instructions[input_idx..].into_iter().cloned().collect());
            break;
        }
    }
    result
}

fn find_input(
    chunks: &[Vec<Instruction>],
    vars: [i64; 4],
    digits: &Vec<i64>,
    dead_ends: &mut HashSet<(usize, i64)>,
    model_number: i64,
) -> Option<i64> {
    if dead_ends.contains(&(chunks.len(), read_var(Z, &vars))) {
        return None;
    }
    let instructions = &chunks[0];
    for d in digits {
        let mut vars = vars.clone();
        let model_number = model_number * 10 + d;
        let z = execute(instructions, *d, &mut vars);
        let chunks = &chunks[1..];
        if chunks.is_empty() {
            if z == 0 {
                return Some(model_number);
            }
        } else if let Some(res) = find_input(chunks, vars, digits, dead_ends, model_number) {
            return Some(res);
        }
    }
    dead_ends.insert((chunks.len(), read_var(Z, &vars)));
    None
}

fn part1(input: &str) -> i64 {
    let instructions = parse(input);
    let instruction_chunks = split_into_chunks(&instructions);
    find_input(
        &instruction_chunks,
        [0; 4],
        &(1..=9).rev().collect(),
        &mut HashSet::new(),
        0,
    )
    .unwrap()
}

fn part2(input: &str) -> i64 {
    let instructions = parse(input);
    let instruction_chunks = split_into_chunks(&instructions);
    find_input(
        &instruction_chunks,
        [0; 4],
        &(1..=9).collect(),
        &mut HashSet::new(),
        0,
    )
    .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("in.txt")), 49917929934999);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("in.txt")), 11911316711816);
}
