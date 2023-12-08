// Problem: https://adventofcode.com/2023/day/8

use std::collections::HashMap;

type Result = usize;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Dir {
    R,
    L,
}

type Input = (Vec<Dir>, HashMap<String, Node>);

fn parse_input(input: &str) -> Input {
    let mut map = HashMap::new();
    let (dirs, nodes) = input.trim().split_once("\n\n").unwrap();
    for line in nodes.lines() {
        let (id, r) = line.split_once(" = ").unwrap();
        let (left, right) = r[1..r.len() - 1].split_once(", ").unwrap();
        map.insert(
            id.to_string(),
            Node {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }
    (
        dirs.chars()
            .map(|ch| match ch {
                'R' => Dir::R,
                'L' => Dir::L,
                _ => panic!(),
            })
            .collect(),
        map,
    )
}

// ------------------------------------------

fn part1((dirs, map): &Input) -> Result {
    let mut cur = "AAA".to_string();
    let mut dirs = dirs.iter().cycle();
    let mut count = 0;
    while cur != "ZZZ" {
        count += 1;
        cur = match dirs.next().unwrap() {
            Dir::R => map[&cur].right.clone(),
            Dir::L => map[&cur].left.clone(),
        };
    }
    count
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test1.txt"));
    assert_eq!(part1(&input), 2);
    let input = parse_input(include_str!("test2.txt"));
    assert_eq!(part1(&input), 6);
}

// ------------------------------------------

fn steps_to_end(cur: &String, dirs: &[Dir], map: &HashMap<String, Node>) -> usize {
    let mut dirs = dirs.iter().cycle();
    let mut cur = cur.to_string();
    let mut count = 0;
    while !cur.ends_with('Z') {
        count += 1;
        let dir = dirs.next().unwrap();
        cur = match dir {
            Dir::R => map[&cur].right.clone(),
            Dir::L => map[&cur].left.clone(),
        };
    }
    count
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn part2((dirs, map): &Input) -> Result {
    let starts = map.keys().filter(|k| k.ends_with('A')).collect::<Vec<_>>();
    let ends = starts
        .iter()
        .map(|start| steps_to_end(start, dirs, map))
        .collect::<Vec<_>>();
    ends.iter().cloned().reduce(lcm).unwrap()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test3.txt"));
    assert_eq!(part2(&input), 6);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
