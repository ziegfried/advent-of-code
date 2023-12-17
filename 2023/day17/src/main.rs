// Problem: https://adventofcode.com/2023/day/17

use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

type Result = u32;
type Point = (i32, i32);
type Map = HashMap<Point, u32>;
type Input = (Map, Point);

fn parse_input(input: &str) -> Input {
    let map = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as i32, j as i32), ch.to_digit(10).unwrap()))
        })
        .collect::<Map>();
    let target_row = *map.keys().map(|(r, _)| r).max().unwrap();
    let target_col = *map.keys().map(|(_, c)| c).max().unwrap();
    (map, (target_row, target_col))
}

// ------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
enum Dir {
    N,
    E,
    S,
    W,
}
use Dir::*;
impl Dir {
    fn right(&self) -> Self {
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }
    fn left(&self) -> Self {
        self.right().right().right()
    }
    fn opposite(&self) -> Self {
        self.right().right()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    pos: Point,
    dir: Dir,
    dir_count: u32,
}

fn next_node(node: &Node, dir: Dir, map: &Map) -> Option<(Node, u32)> {
    let (r, c) = node.pos;
    let pos = match dir {
        N => (r - 1, c),
        E => (r, c + 1),
        S => (r + 1, c),
        W => (r, c - 1),
    };
    map.get(&pos).map(|cost| {
        (
            Node {
                pos,
                dir,
                dir_count: if dir == node.dir {
                    node.dir_count + 1
                } else {
                    1
                },
            },
            *cost,
        )
    })
}

fn successors(node: &Node, map: &Map) -> Vec<(Node, u32)> {
    let dirs = if node.pos == (0, 0) && node.dir_count == 0 {
        vec![next_node(node, E, map), next_node(node, S, map)]
    } else {
        [N, E, S, W]
            .iter()
            .filter(|d| node.dir != d.opposite())
            .filter(|d| node.dir_count < 3 || **d != node.dir)
            .map(|d| next_node(node, *d, map))
            .collect()
    };
    dirs.iter().filter_map(|o| o.clone()).collect()
}

fn part1((map, target): &Input) -> Result {
    let (_, cost) = dijkstra(
        &Node {
            pos: (0, 0),
            dir: N,
            dir_count: 0,
        },
        |node| successors(node, map),
        |node| node.pos == *target,
    )
    .unwrap();
    cost
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 102);
}

// ------------------------------------------

fn successors2(node: &Node, map: &Map) -> Vec<(Node, u32)> {
    let possible_dirs = if node.pos == (0, 0) && node.dir_count == 0 {
        vec![E, S]
    } else if node.dir_count < 4 {
        vec![node.dir]
    } else if node.dir_count >= 10 {
        vec![node.dir.right(), node.dir.left()]
    } else {
        vec![node.dir, node.dir.right(), node.dir.left()]
    };
    possible_dirs
        .iter()
        .filter_map(|d| next_node(node, *d, map))
        .collect()
}

fn part2((map, target): &Input) -> Result {
    let (_, cost) = dijkstra(
        &Node {
            pos: (0, 0),
            dir: N,
            dir_count: 0,
        },
        |node| successors2(node, map),
        |node| node.pos == *target,
    )
    .unwrap();
    cost
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
