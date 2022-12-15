// Problem: https://adventofcode.com/2022/day/15

use std::collections::HashSet;

type Point = (i64, i64);

fn manhattan_distance((x0, y0): Point, (x1, y1): Point) -> i64 {
    (x1 - x0).abs() + (y1 - y0).abs()
}

fn part1(input: &str, target_y: i64) -> usize {
    let input = input
        .trim()
        .lines()
        .map(|line| {
            let (sx, sy, bx, by) = sscanf::scanf!(
                line,
                "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}"
            )
            .unwrap();
            ((sx, sy), (bx, by))
        })
        .collect::<Vec<_>>();

    let mut x_no_beacons = HashSet::<i64>::new();
    let mut x_beacons = HashSet::<i64>::new();

    for (sensor, beacon) in input {
        let dist = manhattan_distance(sensor, beacon);

        if beacon.1 == target_y {
            x_beacons.insert(beacon.0);
            x_no_beacons.remove(&beacon.0);
        }

        let dist_to_y = (sensor.1 - target_y).abs();

        if dist >= dist_to_y {
            let mut dx = 0;
            while manhattan_distance(sensor, (sensor.0 + dx, target_y)) <= dist {
                let a = sensor.0 + dx;
                let b = sensor.0 - dx;
                if !x_beacons.contains(&a) {
                    x_no_beacons.insert(a);
                }
                if !x_beacons.contains(&b) {
                    x_no_beacons.insert(b);
                }
                dx += 1;
            }
        }
    }

    x_no_beacons.len()
}

fn part2(input: &str, min_xy: i64, max_xy: i64) -> i64 {
    let input = input
        .trim()
        .lines()
        .map(|line| {
            let (sx, sy, bx, by) = sscanf::scanf!(
                line,
                "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}"
            )
            .unwrap();
            ((sx, sy), (bx, by))
        })
        .collect::<Vec<_>>();

    let sensors: Vec<(Point, i64)> = input
        .iter()
        .map(|&(s, b)| (s, manhattan_distance(s, b)))
        .collect();

    let is_outside_all = |(x, y): Point| {
        x >= min_xy
            && x <= max_xy
            && y >= min_xy
            && y <= max_xy
            && sensors
                .iter()
                .all(|&(s, d)| manhattan_distance(s, (x, y)) > d)
    };

    for (sensor, dist) in sensors.clone() {
        let (x, y) = sensor;
        for dy in 0..=(dist + 1) {
            let dx = (dist + 1) - dy;
            for p in [
                (x + dx, y + dy),
                (x + dx, y - dy),
                (x - dx, y + dy),
                (x - dx, y - dy),
            ] {
                if is_outside_all(p) {
                    return (p.0 * 4000000) + p.1;
                }
            }
        }
    }
    unreachable!();
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt"), 2000000));
    println!("Part 2: {:?}", part2(include_str!("input.txt"), 0, 4000000));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt"), 10), 26);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt"), 0, 20), 56000011);
}
