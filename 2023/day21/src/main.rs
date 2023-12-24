// Problem: https://adventofcode.com/2023/day/21

use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Range},
};

type Result = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir {
    N,
    E,
    S,
    W,
}
use itertools::Itertools;
use Dir::*;
impl Dir {
    fn to_vec(self) -> Point {
        match self {
            N => Point(-1, 0),
            E => Point(0, 1),
            S => Point(1, 0),
            W => Point(0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i64, i64);
impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

type Input = Map;

struct Map {
    m: HashMap<Point, char>,
    rows: i64,
    cols: i64,
    start: Point,
}
impl Map {
    fn get(&self, Point(r, c): &Point) -> char {
        let r = r % self.rows;
        let r = if r < 0 { r + self.rows } else { r };
        let c = c % self.cols;
        let c = if c < 0 { c + self.cols } else { c };
        self.m[&Point(r, c)]
    }
}

fn parse_input(input: &str) -> Input {
    let mut m = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| (Point(i as i64, j as i64), ch))
        })
        .collect::<HashMap<Point, char>>();

    let start = m.iter().find(|(_, v)| **v == 'S').map(|(k, _)| *k).unwrap();

    m.insert(start, '.');
    let rows = m.keys().map(|Point(i, _)| *i).max().unwrap() + 1;
    let cols = m.keys().map(|Point(_, j)| *j).max().unwrap() + 1;
    Map {
        m,
        rows,
        cols,
        start,
    }
}

fn reachable(map: &Map, steps: usize) -> HashSet<Point> {
    let mut points = HashSet::new();
    points.insert(map.start);
    for step in 0..steps {
        points = next_reachable(&points, map);
        if step == steps - 1 {
            return points;
        }
    }
    unreachable!()
}

fn next_reachable(points: &HashSet<Point>, map: &Map) -> HashSet<Point> {
    let mut next = HashSet::new();
    for p in points {
        for d in [N, E, S, W] {
            let np = *p + d.to_vec();
            let ch = map.get(&np);
            if ch != '#' {
                next.insert(np);
            }
        }
    }
    next
}

fn part1(map: &Input, steps: usize) -> Result {
    reachable(map, steps).len()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input, 6), 16);
    let input = parse_input(include_str!("input.txt"));
    assert_eq!(part1(&input, 64), 3841);
}

fn find_cycle(
    (row_range, col_range): (Range<i64>, Range<i64>),
    map: &Map,
) -> (usize, Point, Vec<usize>) {
    let mut first_point = None;
    let mut first_step = None;
    let mut counts = vec![];
    let mut points: HashSet<Point> = [map.start].iter().cloned().collect();
    let mut prev1 = None;
    let mut prev2 = None;

    for step in 0.. {
        let contained: Vec<Point> = points
            .iter()
            .filter(|Point(r, c)| row_range.contains(r) && col_range.contains(c))
            .cloned()
            .collect::<Vec<_>>();
        let count = contained.len();
        if count > 0 {
            if prev2.is_some() && count == prev2.unwrap() {
                break;
            }
            if first_point.is_none() {
                first_point = Some(contained[0]);
                first_step = Some(step);
            }
            counts.push(count);
            prev2 = prev1;
            prev1 = Some(count);
        }
        points = next_reachable(&points, map);
    }

    (first_step.unwrap(), first_point.unwrap(), counts)
}

fn get_full(step: usize, values: &Vec<usize>) -> usize {
    values[values.len() - if values.len() % 2 == step % 2 { 2 } else { 1 }]
}

fn precalc_tiles(map: &Input) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let (_, _, center) = find_cycle((0..map.rows, 0..map.cols), map);
    let sides = [N, E, S, W]
        .map(|d| d.to_vec())
        .map(|Point(dr, dc)| {
            let rs = dr * map.rows;
            let cs = dc * map.cols;
            let (_, _, counts) = find_cycle(((rs)..(rs + map.rows), cs..(cs + map.cols)), map);
            counts
        })
        .iter()
        .cloned()
        .reduce(|a, b| {
            a.iter()
                .zip(b.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<usize>>()
        })
        .unwrap();
    let diags = [(-1, -1), (-1, 1), (1, 1), (1, -1)]
        .map(|(dr, dc)| {
            let rs = dr * map.rows;
            let cs = dc * map.cols;
            let (_, _, counts) = find_cycle(((rs)..(rs + map.rows), cs..(cs + map.cols)), map);
            counts
        })
        .iter()
        .cloned()
        .reduce(|a, b| {
            a.iter()
                .zip(b.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<usize>>()
        })
        .unwrap();
    (center, sides, diags)
}

#[cfg(test)]
fn zones(points: &HashSet<Point>) -> (usize, usize, usize) {
    let mut center = 0;
    let mut sides = 0;
    let mut diag = 0;
    for Point(r, c) in points {
        let is_center_row = (0..131).contains(r);
        let is_center_col = (0..131).contains(c);
        if is_center_row && is_center_col {
            center += 1;
        } else if is_center_col || is_center_row {
            sides += 1;
        } else {
            diag += 1;
        }
    }
    (center, sides, diag)
}

#[cfg(test)]
fn load_precalc_tiles() -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut it = include_str!("precalc.txt").lines().map(|line| {
        line.split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    });
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}

#[test]
#[ignore]
fn test_calc_reachable_center() {
    let map = parse_input(include_str!("input.txt"));
    let (center, _, _) = load_precalc_tiles();
    let mut points = HashSet::new();
    points.insert(map.start);
    for step in 0..300 {
        let z = zones(&points);
        let c = calc_reachable_center(step, &center);
        assert_eq!(c, z.0, "Step {}", step);
        points = next_reachable(&points, &map);
    }
}

fn calc_reachable_center(step: usize, center: &Vec<usize>) -> usize {
    if step >= center.len() {
        get_full(step, center)
    } else {
        center[step]
    }
}

fn calc_reachable(
    steps: usize,
    center: &Vec<usize>,
    sides: &Vec<usize>,
    diags: &Vec<usize>,
) -> usize {
    calc_reachable_center(steps, center)
        + calc_reachable_sides(steps, sides)
        + calc_reachable_diag(steps, diags)
}

fn get(list: &Vec<usize>, idx: usize) -> usize {
    let l = list.len();
    if idx < l {
        list[idx]
    } else if idx % 2 == l % 2 {
        list[l - 2]
    } else {
        list[l - 1]
    }
}

fn calc_reachable_sides(steps: usize, sides: &Vec<usize>) -> usize {
    let offset = 66;
    if steps < offset {
        return 0;
    }
    let steps = steps - offset;
    let size = 131;

    let full = if steps >= 2 * size {
        let f1 = get_full(steps, sides);
        let n = (steps - size) / size;
        let f2 = get_full(steps - 1, sides);
        f1 * (n / 2 + n % 2) + f2 * (n / 2)
    } else {
        0
    };

    let last = get(sides, steps % size);
    let last2 = if steps >= size {
        get(sides, size + steps % size)
    } else {
        0
    };

    last + last2 + full
}

#[test]
#[ignore]
fn test_calc_reachable_sides() {
    let map = parse_input(include_str!("input.txt"));
    let (_, sides, _) = load_precalc_tiles();
    let mut points = HashSet::new();
    points.insert(map.start);
    for step in 0..1000 {
        let z = zones(&points);
        let s = calc_reachable_sides(step, &sides);
        assert_eq!(s, z.1, "Step {}", step);
        points = next_reachable(&points, &map);
    }
}

fn even_odds_to_n(n: usize) -> (usize, usize) {
    let mut even = 0;
    let mut odd = 0;
    for n in 1..=n {
        if n % 2 == 0 {
            even += n;
        } else {
            odd += n;
        }
    }
    (even, odd)
}

fn calc_reachable_diag(step: usize, diag: &Vec<usize>) -> usize {
    let size = 131;
    let offset = 132;
    if step >= offset {
        let step = step - offset;
        let full = if step >= 2 * size {
            let (even, odd) = even_odds_to_n((step - 2 * size) / size + 1);
            get_full(step + 1, diag) * even + get_full(step, diag) * odd
        } else {
            0
        };
        let last = get(diag, step % size);
        let last2 = if step >= size {
            get(diag, size + step % size)
        } else {
            0
        };
        let n = step / size;
        return full + (last * (n + 1)) + (last2 * n);
    }
    0
}

#[test]
#[ignore]
fn test_calc_reachable_diag() {
    let map = parse_input(include_str!("input.txt"));
    let (_, _, diags) = load_precalc_tiles();
    let mut points = HashSet::new();
    points.insert(map.start);
    for step in 0..1000 {
        let z = zones(&points);
        let s = calc_reachable_diag(step, &diags);
        println!("Step {}: S {:?} -- {}", step, z.2, s);
        assert_eq!(s, z.2, "Step {}", step);
        points = next_reachable(&points, &map);
    }
}

fn part2((center, sides, diags): (Vec<usize>, Vec<usize>, Vec<usize>), steps: usize) -> Result {
    calc_reachable(steps, &center, &sides, &diags)
}

#[test]
fn test_part2() {
    let _input = parse_input(include_str!("input.txt"));
    assert_eq!(part2(load_precalc_tiles(), 26501365), 636391426712747);
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    if std::env::args().any(|arg| arg == "precalc") {
        let (c, s, d) = precalc_tiles(&input);
        for l in [c, s, d] {
            println!("{}", l.iter().join(","));
        }
        return;
    }

    println!("Part 1: {:?}", part1(&input, 64));
    println!("Part 2: {:?}", part2(precalc_tiles(&input), 26501365));
}
