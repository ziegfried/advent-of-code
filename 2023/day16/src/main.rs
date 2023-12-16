// Problem: https://adventofcode.com/2023/day/16

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

type Result = usize;

type Point = (i32, i32);
type Grid = HashMap<Point, char>;

type Input = Grid;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(l, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, ch)| ((l as i32, c as i32), ch))
        })
        .collect::<Grid>()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}
use Dir::*;
impl Dir {
    fn mv(&self, (r, c): &Point) -> Point {
        match self {
            N => (r - 1, *c),
            E => (*r, c + 1),
            S => (r + 1, *c),
            W => (*r, c - 1),
        }
    }
}

// ------------------------------------------

fn part1(input: &Input) -> Result {
    p1(input, ((0, 0), E))
}

fn p1(input: &Input, start: (Point, Dir)) -> Result {
    // let mut cur_pos = (0, 0);
    // let mut cur_dir = E;
    let mut seen = HashSet::new();
    // seen.insert((cur_pos, cur_dir));

    let mut queue = vec![start];

    while let Some((point, dir)) = queue.pop() {
        if seen.contains(&(point, dir)) {
            continue;
        }
        if !input.contains_key(&point) {
            continue;
        }
        seen.insert((point, dir));
        // let next = dir.mv(&point);
        let ch = input[&point];

        match ch {
            '.' => {
                queue.push((dir.mv(&point), dir));
            }
            '\\' => {
                let nd = match dir {
                    N => W,
                    E => S,
                    S => E,
                    W => N,
                };
                queue.push((nd.mv(&point), nd));
            }
            '/' => {
                let nd = match dir {
                    N => E,
                    E => N,
                    S => W,
                    W => S,
                };
                queue.push((nd.mv(&point), nd));
            }
            '|' => match dir {
                N | S => {
                    queue.push((dir.mv(&point), dir));
                }
                E | W => {
                    queue.push((N.mv(&point), N));
                    queue.push((S.mv(&point), S));
                }
            },
            '-' => match dir {
                E | W => {
                    queue.push((dir.mv(&point), dir));
                }
                N | S => {
                    queue.push((E.mv(&point), E));
                    queue.push((W.mv(&point), W));
                }
            },
            _ => {
                panic!("{}", ch);
            }
        }
    }

    seen.iter().map(|(pos, _)| pos).unique().count()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 46);
}

// ------------------------------------------

fn part2(input: &Input) -> Result {
    let max_row = *input.keys().map(|(r, _)| r).max().unwrap();
    let max_col = *input.keys().map(|(_, c)| c).max().unwrap();
    let mut mx = 0;
    for c in 0..=max_col {
        let r = p1(input, ((0, c), S));
        if r > mx {
            mx = r
        };
        let r = p1(input, ((max_row , c), N));
        if r > mx {
            mx = r
        };
    }
    for r in 0..=max_row {
        let v = p1(input, ((r, 0), E));
        if v > mx {
            mx = v
        };
        let v = p1(input, ((r, max_col), W));
        if v > mx {
            mx = v
        };
    }
    mx
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 51);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
