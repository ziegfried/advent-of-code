// Problem: https://adventofcode.com/2022/day/11

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    monkey_no: usize,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, u64},
        multi::separated_list1,
        IResult,
    };

    fn operation_square(input: &str) -> IResult<&str, Operation> {
        let (input, _) = tag("new = old * old")(input)?;
        Ok((input, Operation::Square))
    }

    fn operation_mult(input: &str) -> IResult<&str, Operation> {
        let (input, _) = tag("new = old * ")(input)?;
        let (input, operand) = u64(input)?;
        Ok((input, Operation::Multiply(operand)))
    }

    fn operation_add(input: &str) -> IResult<&str, Operation> {
        let (input, _) = tag("new = old + ")(input)?;
        let (input, operand) = u64(input)?;
        Ok((input, Operation::Add(operand)))
    }

    fn operation(input: &str) -> IResult<&str, Operation> {
        let (input, op) = alt((operation_square, operation_mult, operation_add))(input)?;
        Ok((input, op))
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        let (input, _) = tag("Monkey ")(input)?;
        let (input, monkey_no) = u64(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("  Starting items: ")(input)?;
        let (input, items) = separated_list1(tag(", "), u64)(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("  Operation: ")(input)?;
        let (input, op) = operation(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("  Test: divisible by ")(input)?;
        let (input, test_val) = u64(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("    If true: throw to monkey ")(input)?;
        let (input, if_true) = u64(input)?;
        let (input, _) = newline(input)?;

        let (input, _) = tag("    If false: throw to monkey ")(input)?;
        let (input, if_false) = u64(input)?;

        Ok((
            input,
            Monkey {
                monkey_no: monkey_no as usize,
                items,
                operation: op,
                test: test_val,
                if_true: if_true as usize,
                if_false: if_false as usize,
                inspected: 0,
            },
        ))
    }

    fn double_newline(input: &str) -> IResult<&str, ()> {
        let (input, _) = newline(input)?;
        let (input, _) = newline(input)?;
        Ok((input, ()))
    }

    fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
        let (input, scanners) = separated_list1(double_newline, monkey)(input)?;
        Ok((input, scanners))
    }

    let (input, monkeys) = monkeys(input).unwrap();
    assert_eq!(input.trim(), "");
    monkeys
}

fn part1(input: &str) -> usize {
    let monkeys = parse(input);
    let mut monkey_map: HashMap<usize, Monkey> =
        monkeys.iter().map(|m| (m.monkey_no, m.clone())).collect();
    let monkey_nos: Vec<usize> = monkeys.iter().map(|m| m.monkey_no).collect();

    fn round(monkey_no: usize, monkey_map: &mut HashMap<usize, Monkey>) {
        let (items, monkey) = {
            let monkey = monkey_map.get_mut(&monkey_no).unwrap();
            let items = monkey.items.clone();
            monkey.items = vec![];
            monkey.inspected += items.len();
            (items, monkey.clone())
        };
        for item in items {
            let v = match monkey.operation {
                Operation::Square => item * item,
                Operation::Multiply(m) => item * m,
                Operation::Add(b) => item + b,
            } / 3;

            let push_to = if v % monkey.test == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };

            monkey_map.get_mut(&push_to).unwrap().items.push(v);
        }
    }

    for _ in 0..20 {
        for monkey_no in monkey_nos.iter() {
            round(*monkey_no, &mut monkey_map);
        }
    }

    let mut insp: Vec<usize> = monkey_map.values().map(|m| m.inspected).collect();
    insp.sort();
    insp[insp.len() - 1] * insp[insp.len() - 2]
}

fn part2(input: &str) -> usize {
    let monkeys = parse(input);
    let mut monkey_map: HashMap<usize, Monkey> =
        monkeys.iter().map(|m| (m.monkey_no, m.clone())).collect();
    let monkey_nos: Vec<usize> = monkeys.iter().map(|m| m.monkey_no).collect();

    fn round(monkey_no: usize, monkey_map: &mut HashMap<usize, Monkey>, multiple: u64) {
        let (items, monkey) = {
            let monkey = monkey_map.get_mut(&monkey_no).unwrap();
            let items = monkey.items.clone();
            monkey.items = vec![];
            monkey.inspected += items.len();
            (items, monkey.clone())
        };
        for item in items {
            let v = match monkey.operation {
                Operation::Square => item * item,
                Operation::Multiply(m) => item * m,
                Operation::Add(b) => item + b,
            } % multiple;

            let push_to = if v % monkey.test == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };

            monkey_map.get_mut(&push_to).unwrap().items.push(v);
        }
    }

    let multiple = monkeys.iter().map(|m| m.test).product();

    for _ in 0..10000 {
        for monkey_no in monkey_nos.iter() {
            round(*monkey_no, &mut monkey_map, multiple);
        }
    }

    let mut insp: Vec<usize> = monkey_map.values().map(|m| m.inspected).collect();
    insp.sort();
    insp[insp.len() - 1] * insp[insp.len() - 2]
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 10605);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 2713310158);
}
