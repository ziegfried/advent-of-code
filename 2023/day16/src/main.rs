// Problem: https://adventofcode.com/2023/day/16

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

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

fn step((r, c): &Point, dir: Dir) -> Point {
    match dir {
        N => (r - 1, *c),
        E => (*r, c + 1),
        S => (r + 1, *c),
        W => (*r, c - 1),
    }
}

fn redirect(ch: char, dir: Dir) -> Vec<Dir> {
    match ch {
        '.' => vec![dir],
        '|' => match dir {
            N | S => vec![dir],
            E | W => vec![N, S],
        },
        '-' => match dir {
            N | S => vec![E, W],
            E | W => vec![dir],
        },
        '\\' => vec![match dir {
            N => W,
            E => S,
            S => E,
            W => N,
        }],
        '/' => vec![match dir {
            N => E,
            E => N,
            S => W,
            W => S,
        }],
        _ => panic!("{}", ch),
    }
}

fn energized_particles(input: &Input, start: (Point, Dir)) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from(vec![start]);
    while let Some((point, dir)) = queue.pop_front() {
        seen.insert((point, dir));
        for nd in redirect(input[&point], dir) {
            let next = step(&point, nd);
            if input.contains_key(&next) && !seen.contains(&(next, nd)) {
                queue.push_back((next, nd));
            }
        }
    }
    seen.iter().map(|(pos, _)| pos).unique().count()
}

fn part1(input: &Input) -> Result {
    energized_particles(input, ((0, 0), E))
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 46);
}

fn part2(input: &Input) -> Result {
    let max_row = *input.keys().map(|(r, _)| r).max().unwrap();
    let max_col = *input.keys().map(|(_, c)| c).max().unwrap();
    (0..=max_col)
        .flat_map(|c| vec![((0, c), S), ((max_row, c), N)])
        .chain((0..=max_row).flat_map(|r| vec![((r, 0), E), ((r, max_col), W)]))
        .map(|point| energized_particles(input, point))
        .max()
        .unwrap()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 51);
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
