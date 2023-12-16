// Problem: https://adventofcode.com/2018/day/3

use sscanf::sscanf;
use std::collections::{HashMap, HashSet};

fn parse_line(line: &str) -> (usize, i32, i32, i32, i32) {
    let result = sscanf!(line, "#{usize} @ {i32},{i32}: {i32}x{i32}");
    result.unwrap()
}

fn part1(input: &str) -> usize {
    let claims: Vec<_> = input.lines().map(parse_line).collect();
    let mut map = HashMap::new();
    for (_, x, y, w, h) in claims {
        for dx in 0i32..w {
            for dy in 0i32..h {
                *map.entry((x + dx, y + dy)).or_insert(0) += 1;
            }
        }
    }
    map.values().filter(|&&v| v > 1).count()
}

fn part2(input: &str) -> usize {
    let claims: Vec<_> = input.lines().map(parse_line).collect();
    let mut map = HashMap::new();
    let mut overlaps = HashSet::new();
    for (id, x, y, w, h) in claims.iter() {
        for dx in 0i32..*w {
            for dy in 0i32..*h {
                let pos = (x + dx, y + dy);
                if let Some(prev) = map.get(&pos) {
                    overlaps.insert(id);
                    overlaps.insert(*prev);
                } else {
                    map.insert(pos, id);
                }
            }
        }
    }
    claims
        .iter()
        .map(|c| c.0)
        .find(|id| !overlaps.contains(id))
        .unwrap()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 4);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 3);
}
