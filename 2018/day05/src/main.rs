#![allow(
    dead_code,
    unused_variables,
    unreachable_code,
    clippy::diverging_sub_expression
)]

// Problem: https://adventofcode.com/2018/day/5

use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Unit {
    letter: char,
    polarity: bool,
}

type Result = usize;

type Input = Vec<Unit>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .chars()
        .map(|ch| Unit {
            letter: ch.to_ascii_lowercase(),
            polarity: ch.is_uppercase(),
        })
        .collect()
}

fn react(list: &mut Vec<Unit>) -> bool {
    for i in 1..list.len() {
        let a = list[i - 1];
        let b = list[i];
        if a.letter == b.letter && a.polarity != b.polarity {
            list.remove(i);
            list.remove(i - 1);
            return true;
        }
    }
    false
}

// ------------------------------------------

fn part1(input: &Input) -> Result {
    let mut cur = input.clone();
    while react(&mut cur) {
        //
    }
    cur.len()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 10);
}

// ------------------------------------------

fn part2(input: &Input) -> Result {
    input
        .iter()
        .map(|u| u.letter)
        .collect::<HashSet<_>>()
        .par_iter()
        .map(|l| {
            part1(
                &input
                    .iter()
                    .cloned()
                    .filter(|u| u.letter != *l)
                    .collect::<Vec<Unit>>(),
            )
        })
        .min()
        .unwrap()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 4);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
