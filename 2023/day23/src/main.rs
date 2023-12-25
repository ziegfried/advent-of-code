// Problem: https://adventofcode.com/2023/day/23

use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

type Result = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Add<&Point> for Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}
use Dir::*;
impl Dir {
    fn to_vec(self) -> Point {
        match self {
            N => Point(-1, 0),
            E => Point(0, 1),
            S => Point(1, 0),
            W => Point(0, -1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Node {
    Path,
    Slope(Dir),
}
type Map = HashMap<Point, Node>;
type Input = (Map, Point, Point);

fn parse_input(input: &str) -> Input {
    let map: HashMap<Point, Node> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch != '#')
                .map(move |(j, ch)| {
                    (
                        Point(i as i32, j as i32),
                        match ch {
                            '.' => Node::Path,
                            '>' => Node::Slope(E),
                            '<' => Node::Slope(W),
                            'v' => Node::Slope(S),
                            '^' => Node::Slope(N),
                            _ => panic!(),
                        },
                    )
                })
        })
        .collect();

    let start = map.keys().find(|Point(i, _)| *i == 0).unwrap();
    let last_row = map.keys().map(|Point(i, _)| i).max().unwrap();
    let end = map.keys().find(|Point(i, _)| i == last_row).unwrap();

    (map.clone(), *start, *end)
}

// ------------------------------------------

fn extend_hs(hs: &HashSet<Point>, point: Point) -> HashSet<Point> {
    let mut new = hs.clone();
    new.insert(point);
    new
}

fn find_longest_path(point: Point, seen: HashSet<Point>, map: &Map, target: &Point) -> usize {
    if &point == target {
        return 0;
    }
    let dirs = match map[&point] {
        Node::Path => vec![N, E, S, W],
        Node::Slope(d) => vec![d],
    };

    dirs.iter()
        .map(|d| point + d.to_vec())
        .filter(|p| !seen.contains(p) && map.contains_key(p))
        .map(|p| 1 + find_longest_path(p, extend_hs(&seen, p), map, target))
        .max()
        .unwrap_or_default()
}

fn part1((map, start, end): &Input) -> Result {
    find_longest_path(*start, HashSet::new(), map, end)
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    dbg!(&input);
    assert_eq!(part1(&input), 94);
}

// ------------------------------------------

fn find_longest_path2(
    point: Point,
    len: usize,
    seen: &mut HashSet<Point>,
    map: &Map,
    target: &Point,
) -> Option<usize> {
    if &point == target {
        return Some(len);
    }

    let mut max = 0;
    for d in [N, E, S, W] {
        let next = point + d.to_vec();
        if map.contains_key(&next) && !seen.contains(&next) {
            seen.insert(next);
            if let Some(result) = find_longest_path2(next, len + 1, seen, map, target) {
                if result > max {
                    max = result;
                }
            }
            seen.remove(&next);
        }
    }
    if max == 0 {
        None
    } else {
        Some(max)
    }
}

fn part2((map, start, end): &Input) -> Result {
    find_longest_path2(*start, 0, &mut HashSet::new(), map, end).unwrap()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 154);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
