// Problem: https://adventofcode.com/2022/day/23

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);
impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Point {
    fn row(&self) -> i32 {
        self.0
    }
    fn col(&self) -> i32 {
        self.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
impl Direction {
    fn relative(&self) -> Point {
        match self {
            Direction::N => Point(-1, 0),
            Direction::NE => Point(-1, 1),
            Direction::E => Point(0, 1),
            Direction::SE => Point(1, 1),
            Direction::S => Point(1, 0),
            Direction::SW => Point(1, -1),
            Direction::W => Point(0, -1),
            Direction::NW => Point(-1, -1),
        }
    }
}

fn parse_locations(input: &str) -> HashSet<Point> {
    let mut result = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                result.insert(Point(row as i32, col as i32));
            }
        }
    }
    result
}

fn check_target(p: Point, dir: Direction, points: &HashSet<Point>) -> bool {
    let dirs: [Direction; 3] = match dir {
        N => [NW, N, NE],
        S => [SW, S, SE],
        W => [NW, W, SW],
        E => [NE, E, SE],
        _ => panic!(),
    };
    dirs.iter().all(|d| !points.contains(&(p + d.relative())))
}

fn bounds(elves: &HashSet<Point>) -> ((i32, i32), (i32, i32)) {
    let (min_row, max_row) = elves
        .iter()
        .map(|p| p.row())
        .minmax()
        .into_option()
        .unwrap();
    let (min_col, max_col) = elves
        .iter()
        .map(|p| p.col())
        .minmax()
        .into_option()
        .unwrap();

    ((min_row, max_row), (min_col, max_col))
}

#[allow(dead_code)]
fn print_elves(elves: &HashSet<Point>) -> String {
    let ((min_row, max_row), (min_col, max_col)) = bounds(elves);
    let mut result: String = String::new();
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if elves.contains(&Point(row, col)) {
                result += "#";
            } else {
                result += ".";
            }
        }
        result += "\n";
    }
    result
}

fn move_elves(elves: &HashSet<Point>, target_order: &VecDeque<Direction>) -> HashSet<Point> {
    let mut move_targets: HashMap<Point, usize> = HashMap::new();
    let mut moves = vec![];
    let mut next_elves = HashSet::new();

    for p in elves.iter() {
        if target_order.iter().all(|d| check_target(*p, *d, elves)) {
            next_elves.insert(*p);
        } else {
            let mut found = false;
            for t in target_order.clone() {
                if check_target(*p, t, elves) {
                    *move_targets.entry(*p + t.relative()).or_default() += 1;
                    moves.push((*p, t));
                    found = true;
                    break;
                }
            }
            if !found {
                next_elves.insert(*p);
            }
        }
    }

    for (p, t) in moves {
        let d = p + t.relative();
        if move_targets.get(&d).unwrap() == &1 {
            assert!(!next_elves.contains(&d));
            next_elves.insert(d);
        } else {
            assert!(!next_elves.contains(&p));
            next_elves.insert(p);
        }
    }

    next_elves
}

fn part1(input: &str) -> usize {
    let mut elves = parse_locations(input);
    let mut target_order: VecDeque<Direction> = vec![N, S, W, E].into();
    for _ in 0..10 {
        elves = move_elves(&elves, &target_order);

        let tmp = target_order.pop_front().unwrap();
        target_order.push_back(tmp);
    }

    let b = bounds(&elves);
    let ((min_row, max_row), (min_col, max_col)) = b;
    ((max_row - min_row + 1) as usize * (max_col - min_col + 1) as usize) - elves.len()
}

#[test]
fn test_small() {
    let small = parse_locations(include_str!("small.txt"));
    let small1 = parse_locations(include_str!("small1.txt"));
    let small2 = parse_locations(include_str!("small2.txt"));
    let small3 = parse_locations(include_str!("small3.txt"));

    let calc1 = move_elves(&small, &[N, S, W, E].into());
    assert_eq!(print_elves(&calc1), print_elves(&small1));

    let calc2 = move_elves(&small1, &[S, W, E, N].into());
    assert_eq!(print_elves(&calc2), print_elves(&small2));

    let calc3 = move_elves(&small2, &[W, E, N, S].into());
    assert_eq!(print_elves(&calc3), print_elves(&small3));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 110);
}

fn part2(input: &str) -> usize {
    let mut elves = parse_locations(input);
    let mut target_order: VecDeque<Direction> = vec![N, S, W, E].into();
    for i in 0.. {
        let next = move_elves(&elves, &target_order);
        if next == elves {
            return i + 1;
        }
        elves = next;
        let tmp = target_order.pop_front().unwrap();
        target_order.push_back(tmp);
    }
    unreachable!()
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 20);
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}
