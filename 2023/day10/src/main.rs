// Problem: https://adventofcode.com/2023/day/10

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    N,
    E,
    W,
    S,
}
use Dir::*;
impl Dir {
    fn all() -> [Self; 4] {
        [N, E, S, W]
    }
    fn advance(&self, (l, c): &Point) -> Point {
        match self {
            N => (l - 1, *c),
            E => (*l, c + 1),
            W => (*l, c - 1),
            S => (l + 1, *c),
        }
    }
    fn invert(&self) -> Self {
        match self {
            N => S,
            E => W,
            W => E,
            S => N,
        }
    }
}

type Result = usize;

type Point = (i32, i32);
type Map = HashMap<Point, char>;
type Input = Map;

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
        .collect::<HashMap<Point, char>>()
}

lazy_static! {
    static ref CONNECTORS: HashMap<char, (Dir, Dir)> = {
        let mut map = HashMap::new();
        map.insert('|', (N, S));
        map.insert('-', (W, E));
        map.insert('L', (N, E));
        map.insert('J', (N, W));
        map.insert('7', (S, W));
        map.insert('F', (S, E));
        map
    };
}

fn turn(ch: char, dir: Dir) -> Option<Dir> {
    let entry = dir.invert();
    if let Some(&(a, b)) = CONNECTORS.get(&ch) {
        if entry == a {
            Some(b)
        } else if entry == b {
            Some(a)
        } else {
            None
        }
    } else {
        None
    }
}

fn get_loop(start: Point, dir: Dir, map: &Map) -> Option<(Vec<Point>, Dir)> {
    let mut result = vec![start];
    let mut pos = start;
    let mut dir = dir;
    loop {
        let next = dir.advance(&pos);
        if next == start {
            return Some((result, dir));
        }
        if let Some(ch) = map.get(&next) {
            if let Some(next_dir) = turn(*ch, dir) {
                dir = next_dir;
            } else {
                return None;
            }
            pos = next;
            result.push(next);
        } else {
            return None;
        }
    }
}

fn part1(input: &Input) -> Result {
    let (start, _) = input.iter().find(|&(_, ch)| *ch == 'S').unwrap();
    for dir in Dir::all() {
        if let Some((result, _)) = get_loop(*start, dir, input) {
            return result.len() / 2 + result.len() % 2;
        }
    }
    unreachable!()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test1.txt"));
    assert_eq!(part1(&input), 4);
    let input = parse_input(include_str!("test2.txt"));
    assert_eq!(part1(&input), 8);
    let input = parse_input(include_str!("input.txt"));
    assert_eq!(part1(&input), 6903);
}

fn map_size(map: &Map) -> (i32, i32) {
    let max_line = map.keys().map(|(line, _)| *line).max().unwrap();
    let max_col = map.keys().map(|(_, col)| *col).max().unwrap();
    (max_line + 1, max_col + 1)
}

fn replace_start(start_dir: Dir, end_dir: Dir) -> char {
    *CONNECTORS
        .iter()
        .find(|&(_, v)| *v == (start_dir, end_dir.invert()) || *v == (end_dir.invert(), start_dir))
        .map(|(k, _)| k)
        .unwrap()
}

fn expand_loop(map: &Map, start_dir: Dir, end_dir: Dir, loop_points: &HashSet<Point>) -> Map {
    let mut expanded = HashMap::new();
    for &(l, c) in loop_points {
        let line = l * 3;
        let col = c * 3;
        let ch = map[&(l, c)];
        let ch = if ch == 'S' {
            replace_start(start_dir, end_dir)
        } else {
            ch
        };
        expanded.insert((line, col), ch);
        let (a, b) = CONNECTORS.get(&ch).unwrap();
        for dir in [a, b] {
            expanded.insert(
                match dir {
                    N => (line - 1, col),
                    E => (line, col + 1),
                    W => (line, col - 1),
                    S => (line + 1, col),
                },
                'x',
            );
        }
    }
    expanded
}

fn flood_fill_outside(map: &mut Map) {
    let mut queue = vec![(-1, -1)];
    let (lines, cols) = map_size(map);
    let lines = &(-1..=(lines + 9));
    let cols = &(-1..=(cols + 9));
    while let Some(point) = queue.pop() {
        for dir in Dir::all() {
            let next = dir.advance(&point);
            if lines.contains(&next.0) && cols.contains(&next.1) && !map.contains_key(&next) {
                map.insert(next, 'O');
                queue.push(next);
            }
        }
    }
}

fn part2(input: &Input) -> Result {
    let (lines, cols) = map_size(input);
    let (start, _) = input.iter().find(|&(_, ch)| *ch == 'S').unwrap();
    let (start_dir, (loop_points, end_dir)) = [N, E, S, W]
        .iter()
        .find_map(|dir| get_loop(*start, *dir, input).map(|theloop| (dir, theloop)))
        .unwrap();
    let loop_points = loop_points.iter().cloned().collect::<HashSet<Point>>();
    let mut expanded = expand_loop(input, *start_dir, end_dir, &loop_points);
    flood_fill_outside(&mut expanded);
    (0..lines)
        .cartesian_product(0..cols)
        .filter(|(line, col)| !expanded.contains_key(&(line * 3, col * 3)))
        .count()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test3.txt"));
    assert_eq!(part2(&input), 4);
    let input = parse_input(include_str!("test4.txt"));
    assert_eq!(part2(&input), 8);
    let input = parse_input(include_str!("test5.txt"));
    assert_eq!(part2(&input), 10);
    let input = parse_input(include_str!("input.txt"));
    assert_eq!(part2(&input), 265);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
