// Problem: https://adventofcode.com/2023/day/9

use itertools::Itertools;

type Result = i32;

type Input = Vec<Vec<i32>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn next_for_seq(seq: &Vec<i32>) -> i32 {
    let mut cur = seq;
    let mut diffs;
    let mut next: i32 = 0;
    loop {
        diffs = cur
            .clone()
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<Vec<i32>>();
        if !diffs.is_empty() {
            next += diffs[diffs.len() - 1];
        }
        if diffs.iter().all(|v| *v == 0) {
            break;
        }
        cur = &diffs;
    }
    seq[seq.len() - 1] + next
}

fn part1(input: &Input) -> Result {
    input.iter().map(next_for_seq).sum()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 114);
}

fn part2(input: &Input) -> Result {
    input
        .iter()
        .map(|seq| seq.iter().cloned().rev().collect::<Vec<_>>())
        .map(|seq| next_for_seq(&seq))
        .sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 2);
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
