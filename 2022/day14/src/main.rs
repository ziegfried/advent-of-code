use itertools::Itertools;
use std::collections::HashMap;

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

fn draw_line(grid: &mut HashMap<Point, char>, (x0, y0): Point, (x1, y1): Point) {
    if x0 == x1 {
        for y in (std::cmp::min(y0, y1))..=(std::cmp::max(y0, y1)) {
            grid.insert((x0, y), '#');
        }
    } else if y0 == y1 {
        for x in (std::cmp::min(x0, x1))..=(std::cmp::max(x0, x1)) {
            grid.insert((x, y0), '#');
        }
    } else {
        panic!("{},{} -> {},{}", x0, y0, x1, y1);
    }
}

#[allow(unused)]
fn print_grid(grid: &HashMap<Point, char>) {
    let (&min_x, &max_x) = grid.keys().map(|(x, _)| x).minmax().into_option().unwrap();
    let (&min_y, &max_y) = grid.keys().map(|(_, y)| y).minmax().into_option().unwrap();

    println!();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match grid.get(&(x, y)) {
                Some(ch) => print!("{}", ch),
                None => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn is_settled(p: Point, grid: &HashMap<Point, char>) -> bool {
    grid.contains_key(&down(p))
        && grid.contains_key(&down(left(p)))
        && grid.contains_key(&down(right(p)))
}

fn drop_sand(sand_corn: Point, grid: &mut HashMap<Point, char>, abyss: i32) -> bool {
    if grid.contains_key(&sand_corn) {
        return false;
    }
    let mut cur = sand_corn;

    while cur.1 < abyss && !is_settled(cur, grid) {
        let next = down(cur);

        if grid.contains_key(&next) {
            if !grid.contains_key(&left(next)) {
                cur = left(next);
                continue;
            }
            if !grid.contains_key(&right(next)) {
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
        grid.insert(cur, 'o');
        true
    }
}

fn part1(input: &str) -> usize {
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

    let mut grid = HashMap::new();

    let sand_origin = (500, 0);

    for structure in structures.clone() {
        for i in 1..structure.len() {
            draw_line(&mut grid, structure[i - 1], structure[i]);
        }
    }

    let abyss = structures
        .iter()
        .flat_map(|s| s.iter().map(|(_, y)| y))
        .max()
        .unwrap();

    let mut count = 0;
    loop {
        // print_grid(&grid);
        if drop_sand(sand_origin, &mut grid, *abyss + 1) {
            count += 1;
        } else {
            break;
        }
    }

    count
}

fn part2(input: &str) -> usize {
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

    let mut grid = HashMap::new();

    for structure in structures.clone() {
        for i in 1..structure.len() {
            draw_line(&mut grid, structure[i - 1], structure[i]);
        }
    }

    let sand_origin = (500, 0);
    let abyss = structures
        .iter()
        .flat_map(|s| s.iter().map(|(_, y)| y))
        .max()
        .unwrap();

    let mut count = 0;
    loop {
        // print_grid(&grid);

        // hacky, I know
        let (min_x, max_x) = grid.keys().map(|(x, _)| x).minmax().into_option().unwrap();

        for x in (min_x - 2)..(max_x + 2) {
            grid.insert((x, abyss + 2), '#');
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
