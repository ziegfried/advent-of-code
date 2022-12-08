// Problem: https://adventofcode.com/2022/day/8

use std::str::FromStr;
use Direction::*;

type Pos = (usize, usize);

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn mv(&self, pos: Pos) -> Pos {
        match self {
            Up => (pos.0 - 1, pos.1),
            Down => (pos.0 + 1, pos.1),
            Left => (pos.0, pos.1 - 1),
            Right => (pos.0, pos.1 + 1),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<u8>>);

impl Grid {
    fn get(&self, pos: Pos) -> u8 {
        self.0[pos.0][pos.1]
    }

    fn is_edge(&self, pos: Pos) -> bool {
        let (rows, cols) = (self.0.len(), self.0[0].len());
        let (r, c) = pos;
        c == 0 || r == 0 || c == cols - 1 || r == rows - 1
    }

    fn all_positions(&self) -> GridPosIterator {
        GridPosIterator {
            row: 0,
            col: 0,
            size: (self.0.len(), self.0[0].len()),
        }
    }
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<u8>> = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| format!("{}", c).parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<_>>();
        Ok(Self(grid))
    }
}

struct GridPosIterator {
    row: usize,
    col: usize,
    size: (usize, usize),
}

impl Iterator for GridPosIterator {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.size.0 {
            return None;
        }
        let pos = (self.row, self.col);
        self.col += 1;
        if self.col == self.size.1 {
            self.row += 1;
            self.col = 0;
        }
        Some(pos)
    }
}

fn is_visible(grid: &Grid, pos: Pos) -> bool {
    if grid.is_edge(pos) {
        return true;
    }
    for &dir in &[Up, Right, Down, Left] {
        let mut cur_pos = pos;
        loop {
            let next_pos = dir.mv(cur_pos);
            if grid.get(next_pos) < grid.get(pos) {
                cur_pos = next_pos;
                if grid.is_edge(cur_pos) {
                    return true;
                }
            } else {
                break;
            }
        }
    }
    false
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.all_positions()
        .filter(|&pos| is_visible(&grid, pos))
        .count()
}

fn scenic_score(grid: &Grid, pos: Pos) -> usize {
    let mut score = 1;
    for &dir in &[Up, Right, Down, Left] {
        let mut cur_pos = pos;
        let mut cur_score = 1;
        loop {
            let next_pos = dir.mv(cur_pos);
            if grid.is_edge(next_pos) {
                break;
            }
            if grid.get(next_pos) < grid.get(pos) {
                cur_pos = next_pos;
                cur_score += 1;
            } else {
                break;
            }
        }
        score *= cur_score;
    }
    score
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.all_positions()
        .filter(|&pos| !grid.is_edge(pos))
        .map(|pos| scenic_score(&grid, pos))
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 21);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 8);
}
