// Problem: https://adventofcode.com/2023/day/14

use std::collections::HashMap;

type Result = usize;

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn tilt(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let l = input.len();
    let mut prev = input.clone();
    loop {
        let mut input = prev.clone();
        for r in (1..l).rev() {
            for c in 0..input[0].len() {
                if input[r][c] == 'O' && input[r - 1][c] == '.' {
                    input[r][c] = '.';
                    input[r - 1][c] = 'O';
                }
            }
        }
        if prev == input {
            break;
        }
        prev = input;
    }
    prev
}

fn part1(input: &Input) -> Result {
    tilt(input)
        .iter()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|c| **c == 'O').count() * (input.len() - i))
        .sum()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 136);
}

fn rotate(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .rev()
                .collect::<Vec<char>>()
        })
        .collect()
}

fn part2(input: &Input) -> Result {
    let cycles = 1000000000;
    let mut seen = HashMap::new();
    let mut dish = input.clone();
    let mut i = 0;
    let mut find_cycle = true;
    while i < cycles {
        for _ in 0..4 {
            dish = tilt(&dish);
            dish = rotate(dish);
        }
        if find_cycle {
            if let Some(idx) = seen.get(&dish) {
                let cycle_size = i - idx;
                let remaining = cycles - i;
                i += cycle_size * (remaining / cycle_size);
                find_cycle = false;
            } else {
                seen.insert(dish.clone(), i);
            }
        }
        i += 1;
    }
    dish.iter()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|c| **c == 'O').count() * (input.len() - i))
        .sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 64);
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
