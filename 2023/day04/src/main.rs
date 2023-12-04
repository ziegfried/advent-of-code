// Problem: https://adventofcode.com/2023/day/4

use sscanf::sscanf;
use std::collections::{HashMap, HashSet};

type Result = usize;

type Input = Vec<(usize, HashSet<i32>, HashSet<i32>)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| sscanf!(line, "Card {String}: {String} | {String}").unwrap())
        .map(|(id, winning_str, have_str)| {
            (
                id.trim().parse::<usize>().unwrap(),
                winning_str
                    .split_whitespace()
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>(),
                have_str
                    .split_whitespace()
                    .map(|v| v.parse::<i32>().unwrap())
                    .collect::<HashSet<i32>>(),
            )
        })
        .collect::<Vec<_>>()
}

// ------------------------------------------

fn part1(input: &Input) -> Result {
    input
        .iter()
        .map(|(_, winning, have)| have.intersection(winning).count())
        .map(|count| {
            if count > 1 {
                2usize.pow(count as u32 - 1)
            } else {
                count
            }
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 13);
}

// ------------------------------------------

fn part2(input: &Input) -> Result {
    let mut scores: HashMap<usize, usize> = HashMap::new();
    for (id, winning, have) in input {
        *scores.entry(*id).or_default() += 1;
        let count = winning.intersection(have).count();
        for i in 1..=count {
            *scores.entry(*id + i).or_default() += *scores.get(id).unwrap();
        }
    }
    scores.values().sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 30);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
