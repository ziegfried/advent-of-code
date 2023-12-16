#![allow(
    dead_code,
    unused_variables,
    unreachable_code,
    clippy::diverging_sub_expression
)]

// Problem: https://adventofcode.com/2018/day/6

use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};

type Result = usize;

type Input = Vec<(i32, i32)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|l| sscanf!(l, "{i32}, {i32}").unwrap())
        .collect()
}

fn manhattan_dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn min_dist_idx((x, y): (i32, i32), list: &[(i32, i32)]) -> Option<usize> {
    let mins = list
        .iter()
        .map(|p| manhattan_dist((x, y), *p))
        .enumerate()
        .min_set_by(|a, b| a.1.cmp(&b.1));

    if mins.len() == 1 {
        Some(mins[0].0)
    } else {
        None
    }
}

// ------------------------------------------

fn part1(input: &Input) -> Result {
    let (min_x, max_x) = input
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = input
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();

    dbg!(min_x, min_y, max_x, max_y);

    let mut infinites = HashSet::new();
    let mut map = HashMap::new();

    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            if let Some(idx) = min_dist_idx((x, y), input) {
                if x < min_x || x > max_x || y < min_y || y > max_y {
                    infinites.insert(idx);
                }
                *map.entry(idx).or_default() += 1;
            }
        }
    }

    map.iter()
        .filter(|(k, _)| !infinites.contains(k))
        .map(|(_, v)| *v)
        .max()
        .unwrap()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    dbg!(&input);
    assert_eq!(part1(&input), 17);
}

// ------------------------------------------

fn part2(input: &Input, max_dist: i32) -> Result {
    let (min_x, max_x) = input
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = input
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();

    let mut total = 0;
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            let mut sum = 0;
            for p in input.iter() {
                sum += manhattan_dist((x, y), *p);
                if sum >= max_dist {
                    break;
                }
            }
            if sum < max_dist {
                total += 1;
            }
        }
    }
    total
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input, 32), 16);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input, 10000));
}
