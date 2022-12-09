// Problem: https://adventofcode.com/2022/day/9

use std::{collections::HashSet, fmt::Debug, ops::Add};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos(i32, i32);

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn dir(c: char) -> Pos {
    match c {
        'D' => Pos(0, -1),
        'U' => Pos(0, 1),
        'R' => Pos(1, 0),
        'L' => Pos(-1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn distance(p1: Pos, p2: Pos) -> i32 {
    std::cmp::max((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs())
}

fn part1(input: &str) -> usize {
    let instructions: Vec<(char, i32)> = input
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(l, r)| (l.chars().next().unwrap(), r.parse::<i32>().unwrap()))
        .collect();

    let mut head = Pos(0, 0);
    let mut tail = Pos(0, 0);
    let mut seen = HashSet::new();
    seen.insert(tail);

    for (d, dist) in instructions {
        for _ in 0..dist {
            let last = head;
            head = head + dir(d);
            if distance(head, tail) > 1 {
                tail = last;
                seen.insert(tail);
            }
        }
    }

    seen.len()
}

fn follow_dist(x: i32) -> i32 {
    match x {
        x if x > 1 => 1,
        x if x < -1 => -1,
        _ => 0,
    }
}

fn part2(input: &str) -> usize {
    let instructions: Vec<(char, i32)> = input
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(l, r)| (l.chars().next().unwrap(), r.parse::<i32>().unwrap()))
        .collect();

    let mut rope = [Pos(0, 0); 10];
    let mut seen = HashSet::new();
    seen.insert(rope[9]);

    for (d, dist) in instructions {
        for _ in 0..dist {
            rope[0] = rope[0] + dir(d);
            let mut last = rope[0];
            rope.iter_mut().skip(1).for_each(|cur| {
                if distance(*cur, last) > 1 {
                    *cur = Pos(
                        last.0 + follow_dist(cur.0 - last.0),
                        last.1 + follow_dist(cur.1 - last.1),
                    );
                }
                last = *cur;
            });
            seen.insert(rope[9]);
        }
    }

    seen.len()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test2.txt")), 36);
}
