mod intcode;
use intcode::IntcodeComputer;
use itertools::Itertools;
use std::{collections::HashMap, slice::Iter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Start,
    Wall,
    Empty,
    OxygenSystem,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;
impl Direction {
    fn iter() -> Iter<'static, Direction> {
        static DIRS: [Direction; 4] = [North, East, South, West];
        DIRS.iter()
    }
    fn opposite(&self) -> Direction {
        match &self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

fn move_coords((x, y): (i64, i64), dir: Direction) -> (i64, i64) {
    match dir {
        North => (x, y + 1),
        South => (x, y - 1),
        West => (x - 1, y),
        East => (x + 1, y),
    }
}

#[derive(Debug, Clone, Copy)]
enum Status {
    HitWall,
    Moved(Square),
}

impl Status {
    fn parse(code: i64) -> Self {
        match code {
            0 => Status::HitWall,
            1 => Status::Moved(Square::Empty),
            2 => Status::Moved(Square::OxygenSystem),
            _ => panic!(),
        }
    }
}

fn step(computer: &mut IntcodeComputer, dir: Direction) -> Status {
    computer.push_input(match dir {
        North => 1,
        South => 2,
        West => 3,
        East => 4,
    });
    Status::parse(computer.run_until_next_output().unwrap())
}

#[allow(unused)]
fn print_map(map: &HashMap<(i64, i64), Square>) {
    let (x0, x1) = map.keys().map(|(x, _)| x).minmax().into_option().unwrap();
    let (y0, y1) = map.keys().map(|(_, y)| y).minmax().into_option().unwrap();
    for y in *y0..=*y1 {
        for x in *x0..=*x1 {
            print!(
                "{}",
                match map.get(&(x, y)) {
                    None => ' ',
                    Some(&Square::Start) => 'X',
                    Some(&Square::Empty) => '.',
                    Some(&Square::Wall) => '█',
                    Some(&Square::OxygenSystem) => 'O',
                }
            )
        }
        println!("");
    }
}

fn crawl(
    computer: &mut IntcodeComputer,
    coords: (i64, i64),
    map: &mut HashMap<(i64, i64), Square>,
) {
    for dir in Direction::iter() {
        let target_coords = move_coords(coords, *dir);
        if map.contains_key(&target_coords) {
            continue;
        }
        match step(computer, *dir) {
            Status::HitWall => {
                map.insert(target_coords, Square::Wall);
            }
            Status::Moved(square) => {
                map.insert(target_coords, square);
                let mut fork = computer.clone();
                crawl(&mut fork, target_coords, map);
                step(computer, dir.opposite());
            }
        }
    }
}

fn solve_maze(
    coords: (i64, i64),
    last: Option<(i64, i64)>,
    map: &HashMap<(i64, i64), Square>,
) -> Option<usize> {
    Direction::iter()
        .filter_map(|dir| {
            let target_coords = move_coords(coords, *dir);
            if last != Some(target_coords) {
                match map.get(&target_coords).unwrap() {
                    &Square::Empty | &Square::Start => {
                        solve_maze(target_coords, Some(coords), map).map(|v| v + 1)
                    }
                    &Square::OxygenSystem => Some(1),
                    &Square::Wall => None,
                }
            } else {
                None
            }
        })
        .min()
}

fn part1(input: &str) -> usize {
    let mut computer = IntcodeComputer::from_str(input);
    let mut map: HashMap<(i64, i64), Square> = HashMap::new();
    map.insert((0, 0), Square::Start);
    crawl(&mut computer, (0, 0), &mut map);
    solve_maze((0, 0), None, &map).unwrap()
}

fn oxygen_spread_time(coords: (i64, i64), map: &mut HashMap<(i64, i64), Square>) -> usize {
    Direction::iter()
        .filter_map(|dir| {
            let target_coords = move_coords(coords, *dir);
            match map.get(&target_coords).unwrap() {
                &Square::Empty | &Square::Start => {
                    map.insert(target_coords, Square::OxygenSystem);
                    Some(1 + oxygen_spread_time(target_coords, map))
                }
                &Square::Wall | &Square::OxygenSystem => None,
            }
        })
        .max()
        .unwrap_or(0)
}

fn part2(input: &str) -> usize {
    let mut computer = IntcodeComputer::from_str(input);
    let mut map: HashMap<(i64, i64), Square> = HashMap::new();
    map.insert((0, 0), Square::Start);
    crawl(&mut computer, (0, 0), &mut map);
    let (start_coords, _) = map
        .iter()
        .find(|(_k, v)| v == &&Square::OxygenSystem)
        .unwrap();
    oxygen_spread_time(*start_coords, &mut map)
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("in.txt")), 266);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("in.txt")), 274);
}
