// Problem: https://adventofcode.com/2023/day/17
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

type Result = usize;
type Point = (i32, i32);
type Input = HashMap<Point, u32>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as i32, j as i32), ch.to_digit(10).unwrap()))
        })
        .collect::<Input>()
}

// ------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn all() -> Vec<Self> {
        use Dir::*;
        vec![N, E, S, W]
    }
    fn rel(&self) -> Point {
        use Dir::*;
        match self {
            N => (-1, 0),
            E => (0, 1),
            S => (1, 0),
            W => (0, -1),
        }
    }
    fn opposite(&self) -> Self {
        use Dir::*;
        match self {
            N => S,
            E => W,
            S => N,
            W => E,
        }
    }
    fn rot90(&self) -> Self {
        use Dir::*;
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }
    fn rot270(&self) -> Self {
        self.rot90().rot90().rot90()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos((i32, i32), Option<Dir>, u32);

fn successors(&Pos((r, c), dir, dir_count): &Pos, map: &Input) -> Vec<(Pos, u32)> {
    Dir::all()
        .iter()
        .filter(|d| {
            if dir_count >= 3 {
                Some(**d) != dir
            } else {
                true
            }
        })
        .filter(|d| Some(d.opposite()) != dir)
        .filter_map(|d| {
            let (dr, dc) = d.rel();
            let p = (r + dr, c + dc);
            map.get(&p).map(|cost| {
                (
                    Pos(p, Some(*d), if Some(*d) == dir { dir_count + 1 } else { 1 }),
                    *cost,
                )
            })
        })
        .collect()
}

fn part1(input: &Input) -> Result {
    let target_row = *input.keys().map(|(r, _)| r).max().unwrap();
    let target_col = *input.keys().map(|(_, c)| c).max().unwrap();
    let (_, cost) = dijkstra(
        &Pos((0, 0), None, 0),
        |p| successors(p, input),
        |p| p.0 == (target_row, target_col),
    )
    .unwrap();
    cost as usize
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 102);
}

// ------------------------------------------

fn successors2(&Pos((r, c), dir, dir_count): &Pos, map: &Input) -> Vec<(Pos, u32)> {
    let possible_dirs = if dir.is_none() {
        Dir::all()
    } else if dir_count < 4 {
        vec![dir.unwrap()]
    } else if dir_count >= 10 {
        let d = dir.unwrap();
        vec![d.rot90(), d.rot270()]
    } else {
        let d = dir.unwrap();
        vec![d, d.rot90(), d.rot270()]
    };
    possible_dirs
        .iter()
        .filter_map(|d| {
            let (dr, dc) = d.rel();
            let p = (r + dr, c + dc);
            map.get(&p).map(|cost| {
                (
                    Pos(p, Some(*d), if Some(*d) == dir { dir_count + 1 } else { 1 }),
                    *cost,
                )
            })
        })
        .collect()
}

fn part2(input: &Input) -> Result {
    let target_row = *input.keys().map(|(r, _)| r).max().unwrap();
    let target_col = *input.keys().map(|(_, c)| c).max().unwrap();
    let (_, cost) = dijkstra(
        &Pos((0, 0), None, 0),
        |p| successors2(p, input),
        |p| p.0 == (target_row, target_col),
    )
    .unwrap();
    cost as usize
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 94);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
