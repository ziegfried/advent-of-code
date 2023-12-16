#![allow(
    dead_code,
    unused_variables,
    unreachable_code,
    clippy::diverging_sub_expression
)]

// Problem: {{url}}

type Result = usize;

type Input = String;

fn parse_input(input: &str) -> Input {
    input.to_string()
}

// ------------------------------------------

fn part1(input: &Input) -> Result {
    todo!("part1")
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    dbg!(&input);
    assert_eq!(part1(&input), todo!());
}

// ------------------------------------------

fn part2(input: &Input) -> Result {
    todo!("part2")
}

#[test]
#[ignore]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), todo!());
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
