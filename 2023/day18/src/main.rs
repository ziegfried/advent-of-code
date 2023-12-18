// Problem: https://adventofcode.com/2023/day/18

use itertools::{Itertools, MinMaxResult};
use sscanf::sscanf;
use std::collections::HashSet;

type Result = i64;
type Point = (i64, i64);
type Instruction = (Dir, i64, String);
type Input = Vec<Instruction>;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}
impl Dir {
    fn mv(&self, (r, c): Point) -> Point {
        match self {
            Up => (r - 1, c),
            Down => (r + 1, c),
            Right => (r, c + 1),
            Left => (r, c - 1),
        }
    }
}
use Dir::*;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (ch, amount, color) = sscanf!(line, "{char} {i64} (#{String})").unwrap();
            (
                match ch {
                    'R' => Right,
                    'D' => Down,
                    'L' => Left,
                    'U' => Up,
                    _ => panic!(),
                },
                amount,
                color,
            )
        })
        .collect()
}

// ------------------------------------------

fn unwrap_minmax<T: Copy>(mmr: MinMaxResult<&T>) -> (T, T) {
    match mmr {
        itertools::MinMaxResult::NoElements => panic!(),
        itertools::MinMaxResult::OneElement(v) => (*v, *v),
        itertools::MinMaxResult::MinMax(min, max) => (*min, *max),
    }
}

fn full_surface(map: &HashSet<Point>) -> usize {
    let (min_r, max_r) = unwrap_minmax(map.iter().map(|(r, _)| r).minmax());
    let (min_c, max_c) = unwrap_minmax(map.iter().map(|(_, c)| c).minmax());
    let mut fill_map = HashSet::new();
    let mut queue = vec![(min_r - 1, min_c - 1)];
    while let Some(point) = queue.pop() {
        for dir in [Up, Down, Left, Right] {
            let next = dir.mv(point);
            if ((min_r - 1)..=(max_r + 1)).contains(&next.0)
                && ((min_c - 1)..=(max_c + 1)).contains(&next.1)
                && !map.contains(&next)
                && !fill_map.contains(&next)
            {
                fill_map.insert(next);
                queue.push(next);
            }
        }
    }
    let mut surface = 0;
    for r in (min_r)..=(max_r) {
        for c in (min_c)..=(max_c) {
            if !fill_map.contains(&(r, c)) {
                surface += 1;
            }
        }
    }
    surface
}

fn part1(input: &Input) -> Result {
    let mut cur = (0, 0);
    let mut map: HashSet<Point> = HashSet::new();
    map.insert(cur);
    for (dir, amount, _) in input {
        for _ in 0..(*amount as usize) {
            cur = dir.mv(cur);
            map.insert(cur);
        }
    }
    full_surface(&map) as i64
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 62);
}

// ------------------------------------------

fn decode(color: &str) -> (i32, Dir) {
    (
        i32::from_str_radix(&color[0..5], 16).unwrap(),
        match &color[5..6] {
            "0" => Right,
            "1" => Down,
            "2" => Left,
            "3" => Up,
            _ => panic!(),
        },
    )
}

fn shoelace(map: &Vec<Point>) -> i64 {
    map.iter()
        .tuple_windows()
        .map(|((r1, c1), (r2, c2))| r1 * c2 - r2 * c1)
        .sum::<i64>()
        .abs()
        / 2
        + map.len() as i64 / 2
        + 1
}

fn part2(input: &Input) -> Result {
    let mut cur = (0, 0);
    let mut map = vec![];
    map.push(cur);
    for (_, _, color) in input {
        let (amount, dir) = decode(color);
        for _ in 0..(amount as usize) {
            cur = dir.mv(cur);
            map.push(cur);
        }
    }
    shoelace(&map)
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 952408144115);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
