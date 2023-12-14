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

// ------------------------------------------

fn part1(input: &Input) -> Result {
    let l = input.len();
    let mut prev = input.clone();
    loop {
        let mut input = prev.clone();
        for r in (1..l).rev() {
            for c in 0..input[0].len() {
                let mut up = 0;
                while up < r && input[r - up][c] == 'O' && input[r - up - 1][c] == '.' {
                    input[r][c] = '.';
                    input[r - 1][c] = 'O';
                    up += 1;
                }
            }
        }
        if prev == input {
            break;
        }
        prev = input;
    }
    let input = prev;

    let mut res = 0;
    for (i, line) in input.iter().enumerate() {
        res += line.iter().filter(|c| **c == 'O').count() * (l - i)
    }
    res
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 136);
}

// ------------------------------------------

fn tilt(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut prev = input.clone();
    let l = input.len();
    loop {
        let mut input = prev.clone();
        for r in (1..l).rev() {
            for c in 0..input[0].len() {
                let mut up = 0;
                while up < r && input[r - up][c] == 'O' && input[r - up - 1][c] == '.' {
                    input[r][c] = '.';
                    input[r - 1][c] = 'O';
                    up += 1;
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

fn rotate(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!v.is_empty());
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
    let mut seen = HashMap::new();
    let mut prev = input.clone();
    let mut i = 0;
    let mut s = true;
    while i < 1000000000 {
        let mut cur = prev.clone();
        for _ in 0..4 {
            cur = tilt(&cur);
            cur = rotate(cur);
        }
        if s {
            if let Some(idx) = seen.get(&cur) {
                let cycle_size = i - idx;
                let remaining = 1000000000 - i;
                i += cycle_size * (remaining / cycle_size);
                s = false;
            } else {
                seen.insert(cur.clone(), i);
            }
        }
        prev = cur;
        i += 1;
    }

    let mut res = 0;
    for (i, line) in prev.iter().enumerate() {
        res += line.iter().filter(|c| **c == 'O').count() * (input.len() - i)
    }
    res
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 64);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
