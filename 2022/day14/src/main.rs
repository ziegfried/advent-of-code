use itertools::Itertools;
use std::{collections::HashMap, ops::RangeInclusive};

// Problem: https://adventofcode.com/2022/day/14

type Point = (i32, i32);

fn down(p: Point) -> Point {
    (p.0, p.1 + 1)
}

fn left(p: Point) -> Point {
    (p.0 - 1, p.1)
}

fn right(p: Point) -> Point {
    (p.0 + 1, p.1)
}

fn draw_line(grid: &mut impl Grid, (x0, y0): Point, (x1, y1): Point) {
    fn range(a: i32, b: i32) -> RangeInclusive<i32> {
        if a > b {
            b..=a
        } else {
            a..=b
        }
    }

    if x0 == x1 {
        for y in range(y0, y1) {
            grid.set((x0, y), '#');
        }
    } else if y0 == y1 {
        for x in range(x0, x1) {
            grid.set((x, y0), '#');
        }
    } else {
        panic!("{},{} -> {},{}", x0, y0, x1, y1);
    }
}

#[allow(unused)]
fn print_grid(grid: &impl Grid) {
    let ((min_x, min_y), (max_x, max_y)) = grid.range();

    println!();
    for y in std::cmp::min(0, min_y)..=max_y {
        for x in min_x..=max_x {
            match grid.get((x, y)) {
                Some(ch) => print!("{}", ch),
                None => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn is_settled(p: Point, grid: &impl Grid) -> bool {
    grid.has(down(p)) && grid.has(down(left(p))) && grid.has(down(right(p)))
}

trait Grid {
    fn has(&self, p: Point) -> bool;
    fn get(&self, p: Point) -> Option<char>;
    fn range(&self) -> (Point, Point);
    fn set(&mut self, p: Point, ch: char);
}

struct Grid1(HashMap<Point, char>);

impl Grid for Grid1 {
    fn has(&self, p: Point) -> bool {
        self.0.contains_key(&p)
    }
    fn set(&mut self, p: Point, ch: char) {
        self.0.insert(p, ch);
    }
    fn range(&self) -> (Point, Point) {
        let (&min_x, &max_x) = self
            .0
            .keys()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (&min_y, &max_y) = self
            .0
            .keys()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        ((min_x, min_y), (max_x, max_y))
    }
    fn get(&self, p: Point) -> Option<char> {
        self.0.get(&p).copied()
    }
}

fn build_grid(input: &str) -> Grid1 {
    let structures: Vec<Vec<Point>> = input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    pair.split(',')
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect();
    let mut grid = Grid1(HashMap::new());
    for structure in structures.clone() {
        for i in 1..structure.len() {
            draw_line(&mut grid, structure[i - 1], structure[i]);
        }
    }
    grid
}

fn drop_sand(sand: Point, grid: &mut impl Grid, abyss: i32) -> bool {
    if grid.has(sand) {
        return false;
    }
    let mut cur = sand;

    while cur.1 < abyss && !is_settled(cur, grid) {
        let next = down(cur);

        if grid.has(next) {
            if !grid.has(left(next)) {
                cur = left(next);
                continue;
            }
            if !grid.has(right(next)) {
                cur = right(next);
                continue;
            }
            unreachable!();
        } else {
            cur = next;
        }
    }

    if cur.1 >= abyss {
        false
    } else {
        grid.set(cur, 'o');
        true
    }
}

fn part1(input: &str) -> usize {
    let print_enabled = std::env::var("PRINT_GRID").is_ok();
    let sand_origin = (500, 0);
    let mut grid = build_grid(input);
    let abyss = grid.range().1 .1 + 1;
    let mut count = 0;
    loop {
        if print_enabled {
            print_grid(&grid);
        }
        if drop_sand(sand_origin, &mut grid, abyss) {
            count += 1;
        } else {
            break;
        }
    }
    count
}

struct Grid2(HashMap<Point, char>, i32);

impl Grid for Grid2 {
    fn has(&self, p: Point) -> bool {
        p.1 == self.1 || self.0.contains_key(&p)
    }
    fn set(&mut self, p: Point, ch: char) {
        self.0.insert(p, ch);
    }
    fn range(&self) -> (Point, Point) {
        let (&min_x, &max_x) = self
            .0
            .keys()
            .map(|(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (&min_y, _) = self
            .0
            .keys()
            .map(|(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        ((min_x, min_y), (max_x, self.1))
    }
    fn get(&self, p: Point) -> Option<char> {
        self.0.get(&p).copied()
    }
}

fn part2(input: &str) -> usize {
    let print_enabled = std::env::var("PRINT_GRID").is_ok();
    let sand_origin = (500, 0);
    let grid = build_grid(input);
    let floor = grid.range().1 .1 + 2;
    let mut grid = Grid2(grid.0, floor);
    let mut count = 0;
    loop {
        if print_enabled {
            print_grid(&grid);
        }
        if drop_sand(sand_origin, &mut grid, i32::MAX) {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 24);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 93);
}
