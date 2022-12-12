// Problem: https://adventofcode.com/2022/day/12

use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn find(grid: &Vec<Vec<u8>>, val: u8) -> Pos {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == val {
                return Pos(row as i32, col as i32);
            }
        }
    }
    panic!();
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
    rows: i32,
    cols: i32,
    start: Pos,
    dest: Pos,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut grid: Vec<Vec<u8>> = input
            .trim()
            .lines()
            .map(|l| l.chars().map(|ch| ch.to_string().as_bytes()[0]).collect())
            .collect();
        let start = find(&grid, b'S');
        let dest = find(&grid, b'E');
        grid[start.0 as usize][start.1 as usize] = b'a';
        grid[dest.0 as usize][dest.1 as usize] = b'z';
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        Self {
            grid,
            rows,
            cols,
            start,
            dest,
        }
    }
    fn get(&self, pos: Pos) -> u8 {
        self.grid[pos.0 as usize][pos.1 as usize]
    }
    fn is_valid(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.rows && pos.1 < self.cols
    }
    fn iter(&self) -> impl Iterator<Item = Pos> {
        (0..self.rows)
            .cartesian_product(0..self.cols)
            .map(|(r, c)| Pos(r as i32, c as i32))
    }
}

fn find_path(start: Pos, grid: &Grid) -> Option<usize> {
    let mut seen = HashSet::<Pos>::new();
    let mut queue = VecDeque::<(Pos, usize)>::new();
    queue.push_back((start, 0));
    while !queue.is_empty() {
        let (pos, cur_len) = queue.pop_front().unwrap();
        if !seen.contains(&pos) {
            if pos == grid.dest {
                return Some(cur_len);
            }
            seen.insert(pos);
            for dir in &[Pos(1, 0), Pos(-1, 0), Pos(0, 1), Pos(0, -1)] {
                let next = pos + *dir;
                if grid.is_valid(next) && grid.get(next) <= grid.get(pos) + 1 {
                    queue.push_back((next, cur_len + 1));
                }
            }
        }
    }
    None
}

fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);
    find_path(grid.start, &grid).unwrap()
}

fn part2(input: &str) -> usize {
    let grid = Grid::parse(input);
    let low_height = b'a';
    grid.iter()
        .filter(|pos| grid.get(*pos) == low_height)
        .filter_map(|pos| find_path(pos, &grid))
        .min()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 31);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 29);
}
