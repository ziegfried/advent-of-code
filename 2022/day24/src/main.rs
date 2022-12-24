// Problem: https://adventofcode.com/2022/day/24

use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
};
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '>' => Right,
            '<' => Left,
            'v' => Down,
            '^' => Up,
            _ => panic!("invalid dir {}", c),
        }
    }
    fn go(&self, &Point(row, col): &Point) -> Point {
        match self {
            Up => Point(row - 1, col),
            Right => Point(row, col + 1),
            Down => Point(row + 1, col),
            Left => Point(row, col - 1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Bounds(Point, Point);

impl Bounds {
    fn contains(&self, p: Point) -> bool {
        let Point(r0, c0) = self.0;
        let Point(r1, c1) = self.1;
        let r = p.row();
        let c = p.col();
        r > r0 && r < r1 - 1 && c > c0 && c < c1 - 1
    }

    fn width(&self) -> i32 {
        self.1 .1 - self.0 .1
    }
    fn height(&self) -> i32 {
        self.1 .0 - self.0 .0
    }
}

#[derive(Debug)]
struct Map {
    blizzards: Vec<(Point, Direction)>,
    bounds: Bounds,
    entry: Point,
    exit: Point,
}
impl Map {
    fn get_blizzards_at(&self, minute: usize) -> HashSet<Point> {
        self.blizzards
            .iter()
            .map(|(p, d)| blizzard_at_time(*p, *d, minute, &self.bounds))
            .collect()
    }
}

fn blizzard_at_time(start: Point, dir: Direction, minute: usize, bounds: &Bounds) -> Point {
    match dir {
        Right => Point(
            start.row(),
            (start.col() - 1 + minute as i32) % (bounds.width() - 2) + 1,
        ),
        Left => Point(
            start.row(),
            (start.col() - 1 - minute as i32).rem_euclid(bounds.width() - 2) + 1,
        ),
        Up => Point(
            (start.row() - 1 - minute as i32).rem_euclid(bounds.height() - 2) + 1,
            start.col(),
        ),
        Down => Point(
            (start.row() - 1 + minute as i32) % (bounds.height() - 2) + 1,
            start.col(),
        ),
    }
}

fn parse_map(input: &str) -> Map {
    let lines: Vec<&str> = input.trim().lines().collect();
    let first = lines[0];
    let width = first.len();
    let height = lines.len();
    let last = lines[lines.len() - 1];
    let entry_col = first.chars().position(|c| c == '.').unwrap();
    let exit_col = last.chars().position(|c| c == '.').unwrap();
    let mut blizzards = vec![];
    (1..lines.len() - 1).for_each(|row| {
        let line = lines[row];
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' && ch != '#' {
                blizzards.push((Point(row as i32, col as i32), Direction::from_char(ch)));
            }
        }
    });
    Map {
        blizzards,
        bounds: Bounds(Point(0, 0), Point(height as i32, width as i32)),
        entry: Point(0, entry_col as i32),
        exit: Point(lines.len() as i32 - 1, exit_col as i32),
    }
}

fn find_best(map: &Map, start_minute: usize, start: Point, end: Point) -> usize {
    let mut best = usize::MAX;
    let mut queue = VecDeque::new();
    queue.push_back((start, start_minute));
    let mut seen = HashSet::new();
    while !queue.is_empty() {
        let (pos, minute) = queue.pop_front().unwrap();
        if minute >= best {
            continue;
        }
        if seen.contains(&(pos, minute)) {
            continue;
        }
        seen.insert((pos, minute));
        let next_blizzards = map.get_blizzards_at(minute);
        for d in [Up, Down, Left, Right] {
            let n = d.go(&pos);
            if n == end && minute < best {
                best = minute;
                continue;
            }
            if map.bounds.contains(n) && !next_blizzards.contains(&n) {
                queue.push_back((n, minute + 1));
            }
        }
        if !next_blizzards.contains(&pos) {
            queue.push_back((pos, minute + 1));
        }
    }
    best
}

fn part1(map: &Map) -> usize {
    find_best(map, 0, map.entry, map.exit)
}

fn part2(map: &Map, p1: usize) -> usize {
    let back_at_start = find_best(map, p1, map.exit, map.entry);
    find_best(map, back_at_start, map.entry, map.exit)
}

fn main() {
    let map = parse_map(include_str!("input.txt"));
    let p1 = part1(&map);
    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", part2(&map, p1));
}

#[test]
fn test_part1() {
    assert_eq!(part1(&parse_map(include_str!("test.txt"))), 18);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse_map(include_str!("test.txt")), 18), 54);
}
