mod intcode;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::{cmp, ops};

use intcode::{parse_program, IntcodeComputer};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
struct Point(i32, i32);
impl Eq for Point {}
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        if turn == Turn::Left {
            return (0..3).fold(self.clone(), |d, _| d.turn(Turn::Right));
        }
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn run_robot_program(input: &str, hull: &mut HashMap<Point, Color>) {
    let program = parse_program(input);
    let mut robot_loc = Point(0, 0);
    let mut robot_dir = Direction::North;
    let mut computer = IntcodeComputer::create(program, vec![]);

    loop {
        let cur_color: Color = hull.get(&robot_loc).unwrap_or(&Color::Black).clone();
        computer.add_input(match cur_color {
            Color::Black => 0,
            Color::White => 1,
        });
        let next_color = match computer.run_until_next_output() {
            Some(0) => Color::Black,
            Some(1) => Color::White,
            Some(v) => panic!("expected color, instead got {}", v),
            _ => break,
        };
        let turn = match computer.run_until_next_output() {
            Some(0) => Turn::Left,
            Some(1) => Turn::Right,
            Some(v) => panic!(
                "expected turn direction 0=left or 1=right, instead got {}",
                v
            ),
            _ => panic!("expected turn direction, got halt instead"),
        };
        hull.insert(robot_loc, next_color);
        robot_dir = robot_dir.turn(turn);
        robot_loc = robot_loc
            + match robot_dir {
                Direction::North => Point(-1, 0),
                Direction::East => Point(0, 1),
                Direction::South => Point(1, 0),
                Direction::West => Point(0, -1),
            };
    }
}

fn part1(input: &str) -> usize {
    let mut hull = HashMap::<Point, Color>::new();
    run_robot_program(input, &mut hull);
    hull.len()
}

fn extend_range(a: RangeInclusive<i32>, b: RangeInclusive<i32>) -> RangeInclusive<i32> {
    RangeInclusive::new(
        cmp::min(*a.start(), *b.start()),
        cmp::max(*a.end(), *b.end()),
    )
}

fn part2(input: &str) -> String {
    let mut hull = HashMap::<Point, Color>::new();
    hull.insert(Point(0, 0), Color::White);
    run_robot_program(input, &mut hull);
    let x_range = hull
        .keys()
        .map(|Point(x, _)| *x..=*x)
        .reduce(extend_range)
        .unwrap();
    let y_range = hull
        .keys()
        .map(|Point(_, y)| *y..=*y)
        .reduce(extend_range)
        .unwrap();
    let hull_ref = &hull;
    x_range
        .map(|x| {
            y_range
                .clone()
                .map(move |y| match hull_ref.get(&Point(x, y)) {
                    Some(Color::White) => '█',
                    Some(Color::Black) => '.',
                    None => ' ',
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2:\n\n{}\n", part2(include_str!("in.txt")));
}
