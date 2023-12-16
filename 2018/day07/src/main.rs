#![allow(
    dead_code,
    unused_variables,
    unreachable_code,
    clippy::diverging_sub_expression
)]

// Problem: https://adventofcode.com/2018/day/7

use sscanf::sscanf;
use std::collections::{HashMap, HashSet};

type Input = Vec<(char, char)>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            sscanf!(
                line,
                "Step {char} must be finished before step {char} can begin."
            )
            .unwrap()
        })
        .collect()
}

// ------------------------------------------

fn part1(input: &Input) -> String {
    let mut all = HashSet::new();
    let mut deps = HashMap::new();
    for (a, b) in input.iter() {
        deps.entry(*b).or_insert(vec![]).push(*a);
        all.insert(*a);
        all.insert(*b);
    }

    let start = all.iter().find(|v| !deps.contains_key(v)).unwrap();
    let mut remaining = all.clone();
    let mut result: Vec<char> = vec![*start];
    let mut resolved = HashSet::new();
    resolved.insert(*start);
    remaining.remove(start);

    while !remaining.is_empty() {
        let next = remaining
            .iter()
            .filter(|c| {
                if let Some(d) = deps.get(c) {
                    d.iter().all(|o| resolved.contains(o))
                } else {
                    true
                }
            })
            .cloned()
            .min()
            .unwrap();

        remaining.remove(&next);
        resolved.insert(next);
        result.push(next);
    }
    result.iter().cloned().collect()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), "CABDFE".to_string());
}

// ------------------------------------------

fn part2(input: &Input, worker_count: usize, base_duration: usize) -> usize {
    let mut all = HashSet::new();
    let mut deps = HashMap::new();
    for (a, b) in input.iter() {
        deps.entry(*b).or_insert(vec![]).push(*a);
        all.insert(*a);
        all.insert(*b);
    }

    let mut resolved = HashSet::new();
    let mut workers: Vec<(char, usize)> = vec![];
    let mut seconds = 0;
    loop {
        if workers.is_empty() && resolved.len() == all.len() {
            break;
        }
        while workers.len() < worker_count {
            if let Some(next) = all
                .iter()
                .filter(|c| !resolved.contains(*c) && !workers.iter().any(|(x, _)| *c == x))
                .filter(|c| {
                    if let Some(d) = deps.get(c) {
                        d.iter().all(|o| resolved.contains(o))
                    } else {
                        true
                    }
                })
                .cloned()
                .min()
            {
                workers.push((next, next as usize - 'A' as usize + 1 + base_duration));
            } else {
                break;
            }
        }
        for entry in workers.iter_mut() {
            entry.1 -= 1;
        }
        for i in (0..workers.len()).rev() {
            if workers[i].1 == 0 {
                resolved.insert(workers[i].0);
                workers.remove(i);
            }
        }
        seconds += 1;
    }
    seconds
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input, 2, 0), 15);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input, 5, 60));
}
