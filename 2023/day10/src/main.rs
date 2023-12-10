// Problem: https://adventofcode.com/2023/day/10

use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    N,
    E,
    W,
    S,
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

fn move_in_dir((l, c): Point, dir: Dir) -> Point {
    match dir {
        Dir::N => (l - 1, c),
        Dir::E => (l, c + 1),
        Dir::W => (l, c - 1),
        Dir::S => (l + 1, c),
    }
}

fn turn(ch: char, dir: Dir) -> Option<Dir> {
    match ch {
        '.' => None,
        // | is a vertical pipe connecting north and south.
        '|' => match dir {
            Dir::N => Some(Dir::N),
            Dir::S => Some(Dir::S),
            Dir::E => None,
            Dir::W => None,
        },
        // - is a horizontal pipe connecting east and west.
        '-' => match dir {
            Dir::E => Some(Dir::E),
            Dir::W => Some(Dir::W),
            Dir::N => None,
            Dir::S => None,
        },
        // L is a 90-degree bend connecting north and east.
        'L' => match dir {
            Dir::N => None,
            Dir::E => None,
            Dir::W => Some(Dir::N),
            Dir::S => Some(Dir::E),
        },
        // J is a 90-degree bend connecting north and west.
        'J' => match dir {
            Dir::N => None,
            Dir::E => Some(Dir::N),
            Dir::W => None,
            Dir::S => Some(Dir::W),
        },
        // 7 is a 90-degree bend connecting south and west.
        '7' => match dir {
            Dir::N => Some(Dir::W),
            Dir::E => Some(Dir::S),
            Dir::W => None,
            Dir::S => None,
        },
        // F is a 90-degree bend connecting south and east.
        'F' => match dir {
            Dir::N => Some(Dir::E),
            Dir::E => None,
            Dir::W => Some(Dir::S),
            Dir::S => None,
        },
        _ => panic!("invalid dir"),
    }
}

fn get_max_dist(start: Point, dir: Dir, map: &Map) -> Option<usize> {
    let mut dist_from_start = 0;
    let mut pos = start;
    let mut dir = dir;

    loop {
        let next = move_in_dir(pos, dir);
        if next == start {
            return Some(dist_from_start / 2 + dist_from_start % 2);
        }
        if let Some(ch) = map.get(&next) {
            dist_from_start += 1;
            if let Some(next_dir) = turn(*ch, dir) {
                dir = next_dir;
            } else {
                return None;
            }

            pos = next;
        } else {
            return None;
        }
    }
}

fn part1(input: &Input) -> Result {
    let (start, _) = input.iter().find(|&(_, ch)| *ch == 'S').unwrap();
    for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
        if let Some(result) = get_max_dist(*start, dir, input) {
            return result;
        }
    }
    unreachable!()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 4);
    let input = parse_input(include_str!("test2.txt"));
    assert_eq!(part1(&input), 8);
    let input = parse_input(include_str!("input.txt"));
    assert_eq!(part1(&input), 6903);
}

// ------------------------------------------

fn get_loop(start: Point, dir: Dir, map: &Map) -> Option<(Vec<Point>, Dir)> {
    let mut result = vec![start];
    let mut pos = start;
    let mut dir = dir;
    loop {
        let next = move_in_dir(pos, dir);
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

fn flood_fill_outside(map: &mut Map) {
    use Dir::*;
    let mut queue = vec![(-1, -1)];
    let (lines, cols) = map_size(map);
    let lines = &(-1..=(lines + 9));
    let cols = &(-1..=(cols + 9));
    while let Some(point) = queue.pop() {
        for dir in [N, E, S, W] {
            let next = move_in_dir(point, dir);
            if lines.contains(&next.0) && cols.contains(&next.1) && !map.contains_key(&next) {
                map.insert(next, 'O');
                queue.push(next);
            }
        }
    }
}

fn invert(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::S,
        Dir::E => Dir::W,
        Dir::W => Dir::E,
        Dir::S => Dir::N,
    }
}

fn replace_start(start_dir: Dir, end_dir: Dir) -> char {
    match (start_dir, invert(end_dir)) {
        (Dir::N, Dir::E) => 'L',
        (Dir::N, Dir::W) => 'J',
        (Dir::N, Dir::S) => '|',
        (Dir::E, Dir::N) => 'L',
        (Dir::E, Dir::W) => '-',
        (Dir::E, Dir::S) => 'F',
        (Dir::W, Dir::N) => 'J',
        (Dir::W, Dir::E) => '-',
        (Dir::W, Dir::S) => '7',
        (Dir::S, Dir::N) => '|',
        (Dir::S, Dir::E) => 'F',
        (Dir::S, Dir::W) => '7',
        _ => panic!(),
    }
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
        match ch {
            '|' => {
                expanded.insert((line - 1, col), '|');
                expanded.insert((line + 1, col), '|');
            }
            '-' => {
                expanded.insert((line, col - 1), '-');
                expanded.insert((line, col + 1), '-');
            }
            'F' => {
                expanded.insert((line, col + 1), '-');
                expanded.insert((line + 1, col), '|');
            }
            'J' => {
                expanded.insert((line - 1, col), '|');
                expanded.insert((line, col - 1), '-');
            }
            'L' => {
                expanded.insert((line - 1, col), '|');
                expanded.insert((line, col + 1), '-');
            }
            '7' => {
                expanded.insert((line + 1, col), '|');
                expanded.insert((line, col - 1), '-');
            }
            _ => {
                //
            }
        }
    }
    expanded
}

fn map_size(map: &Map) -> (i32, i32) {
    let max_line = map.keys().map(|(line, _)| *line).max().unwrap();
    let max_col = map.keys().map(|(_, col)| *col).max().unwrap();
    (max_line + 1, max_col + 1)
}

fn part2(input: &Input) -> Result {
    use Dir::*;
    let (lines, cols) = map_size(input);
    let (start, _) = input.iter().find(|&(_, ch)| *ch == 'S').unwrap();
    let (start_dir, (loop_points, end_dir)) = [N, E, S, W]
        .iter()
        .find_map(|dir| get_loop(*start, *dir, input).map(|theloop| (dir, theloop)))
        .unwrap();
    let loop_points = loop_points.iter().cloned().collect::<HashSet<Point>>();
    let mut expanded = expand_loop(input, *start_dir, end_dir, &loop_points);
    flood_fill_outside(&mut expanded);
    let mut count = 0;
    for line in 0..lines {
        for col in 0..cols {
            if !expanded.contains_key(&(line * 3, col * 3)) {
                count += 1;
            }
        }
    }
    count
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
