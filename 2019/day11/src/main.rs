mod intcode;
use std::cmp;
use std::collections::HashMap;

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

type Point = (i32, i32);

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

fn add(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn move_in(direction: Direction) -> Point {
    match direction {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    }
}

fn run_robot_program(input: &str, hull: &mut HashMap<Point, Color>) {
    let program = parse_program(input);
    let mut robot_loc: Point = (0, 0);
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
        robot_loc = add(robot_loc, move_in(robot_dir));
    }
}

fn part1(input: &str) -> usize {
    let mut hull = HashMap::<Point, Color>::new();
    run_robot_program(input, &mut hull);
    hull.len()
}

fn part2(input: &str) -> String {
    let mut hull = HashMap::<Point, Color>::new();
    hull.insert((0, 0), Color::White);
    run_robot_program(input, &mut hull);

    let mut min_x: i32 = i32::MAX;
    let mut min_y: i32 = i32::MAX;
    let mut max_x: i32 = i32::MIN;
    let mut max_y: i32 = i32::MIN;

    for (x, y) in hull.keys() {
        min_x = cmp::min(*x, min_x);
        min_y = cmp::min(*y, min_y);
        max_x = cmp::max(*x, max_x);
        max_y = cmp::max(*y, max_y);
    }

    let hull_ref = &hull;
    (min_x..=max_x)
        .map(|x| {
            (min_y..=max_y)
                .map(move |y| match hull_ref.get(&(x, y)) {
                    Some(Color::White) => '#',
                    Some(Color::Black) => ' ',
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
