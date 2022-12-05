use itertools::Itertools;

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut stacks = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|_| Vec::new())
        .collect::<Vec<Vec<char>>>();
    for line in lines {
        (0..stacks.len()).for_each(|stack_idx| {
            let char_idx = stack_idx * 3 + stack_idx + 1;
            let s = &line[char_idx..char_idx + 1];
            if s != " " {
                stacks[stack_idx].push(s.chars().next().unwrap());
            }
        });
    }
    stacks
        .iter()
        .map(|s| s.iter().copied().rev().collect())
        .collect()
}

fn part1(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    for line in instructions.lines() {
        let (_, amount, _, src, _, dest) = line.split_whitespace().collect_tuple().unwrap();
        let amount: u32 = amount.parse().unwrap();
        let src: usize = src.parse().unwrap();
        let dest: usize = dest.parse().unwrap();

        let mut moving = vec![];
        for _ in 0..amount {
            let el = stacks[src - 1].pop().unwrap();
            moving.push(el);
        }
        for el in moving.into_iter() {
            stacks[dest - 1].push(el);
        }
    }
    stacks.iter().map(|s| s[s.len() - 1]).collect()
}

fn part2(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks);
    for line in instructions.lines() {
        let (_, amount, _, src, _, dest) = line.split_whitespace().collect_tuple().unwrap();

        let amount: u32 = amount.parse().unwrap();
        let src: usize = src.parse().unwrap();
        let dest: usize = dest.parse().unwrap();

        let mut moving = vec![];
        for _ in 0..amount {
            let el = stacks[src - 1].pop().unwrap();
            moving.push(el);
        }
        for el in moving.into_iter().rev() {
            stacks[dest - 1].push(el);
        }
    }
    stacks.iter().map(|s| s[s.len() - 1]).collect()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), "CMZ");
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), "MCD");
}
