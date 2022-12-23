// Problem: https://adventofcode.com/2022/day/18

use itertools::Itertools;
use std::collections::HashSet;

type Point = (i32, i32, i32);

fn parse_point(line: &str) -> Point {
    let v: Vec<_> = line.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
    (v[0], v[1], v[2])
}

fn connected((x1, y1, z1): Point, (x2, y2, z2): Point) -> bool {
    let dx = x1 - x2;
    let dy = y1 - y2;
    let dz = z1 - z2;
    (dx.abs() == 1 && dy == 0 && dz == 0)
        || (dy.abs() == 1 && dx == 0 && dz == 0)
        || (dz.abs() == 1 && dx == 0 && dy == 0)
}

fn surface(cubes: &HashSet<Point>) -> usize {
    let mut count: usize = 6 * cubes.len();
    for &c1 in cubes.iter() {
        for &c2 in cubes.iter() {
            if connected(c1, c2) {
                count -= 1;
            }
        }
    }
    count
}

fn part1(input: &str) -> usize {
    let cubes: HashSet<Point> = input.trim().lines().map(parse_point).collect();
    surface(&cubes)
}

fn neighbors((x, y, z): (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
    vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
    .into_iter()
}

type Bounds = ((i32, i32), (i32, i32), (i32, i32));

fn bounds(cubes: &HashSet<Point>) -> Bounds {
    (
        cubes
            .iter()
            .map(|(x, _, _)| *x)
            .minmax()
            .into_option()
            .unwrap(),
        cubes
            .iter()
            .map(|(_, y, _)| *y)
            .minmax()
            .into_option()
            .unwrap(),
        cubes
            .iter()
            .map(|(_, _, z)| *z)
            .minmax()
            .into_option()
            .unwrap(),
    )
}

fn extend_bounds(((min_x, max_x), (min_y, max_y), (min_z, max_z)): Bounds, amount: i32) -> Bounds {
    (
        (min_x - amount, max_x + amount),
        (min_y - amount, max_y + amount),
        (min_z - amount, max_z + amount),
    )
}

fn is_inside_bounds(
    (x, y, z): Point,
    ((min_x, max_x), (min_y, max_y), (min_z, max_z)): Bounds,
) -> bool {
    x >= min_x && x <= max_x && y >= min_y && y <= max_y && z >= min_z && z <= max_z
}

fn fill_space_in_bounds(cubes: &mut HashSet<Point>, bounds: Bounds) {
    let ((min_x, _), (min_y, _), (min_z, _)) = bounds;
    let mut queue = vec![(min_x, min_y, min_z)];
    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        cubes.insert(p);
        for n in neighbors(p) {
            if is_inside_bounds(n, bounds) && !cubes.contains(&n) {
                queue.push(n);
            }
        }
    }
}

fn outside_surface(((min_x, max_x), (min_y, max_y), (min_z, max_z)): Bounds) -> i32 {
    let x = max_x - min_x + 1;
    let y = max_y - min_y + 1;
    let z = max_z - min_z + 1;
    2 * x * y + 2 * x * z + 2 * y * z
}

fn part2(input: &str) -> usize {
    let total = part1(input);
    let mut cubes: HashSet<(i32, i32, i32)> = input.trim().lines().map(parse_point).collect();
    let bounds = extend_bounds(bounds(&cubes), 1);
    fill_space_in_bounds(&mut cubes, bounds);
    let surface = surface(&cubes);
    total - (surface - outside_surface(bounds) as usize)
}

fn main() {
    println!("Part 1: {}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 64);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 58);
}
