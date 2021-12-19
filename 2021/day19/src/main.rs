use std::collections::{HashMap, HashSet};
use itertools::Itertools;

type Position = (i32, i32, i32);
type Orientation = (i32, i32, i32, usize, usize, usize);

fn parse(input: &str) -> Vec<(usize, Vec<Position>)> {
    use nom::{
        bytes::complete::tag,
        character::complete::{i32, newline},
        multi::separated_list1,
        IResult,
    };

    fn point(input: &str) -> IResult<&str, Position> {
        let (input, x) = i32(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = i32(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, z) = i32(input)?;
        return Ok((input, (x, y, z)));
    }

    fn scanner(input: &str) -> IResult<&str, (usize, Vec<Position>)> {
        let (input, _) = tag("--- scanner ")(input)?;
        let (input, scanner_no) = i32(input)?;
        let (input, _) = tag(" ---")(input)?;
        let (input, _) = newline(input)?;
        let (input, points) = separated_list1(newline, point)(input)?;
        Ok((input, (scanner_no as usize, points)))
    }

    fn double_newline(input: &str) -> IResult<&str, ()> {
        let (input, _) = newline(input)?;
        let (input, _) = newline(input)?;
        Ok((input, ()))
    }

    fn scanners(input: &str) -> IResult<&str, Vec<(usize, Vec<Position>)>> {
        let (input, scanners) = separated_list1(double_newline, scanner)(input)?;
        Ok((input, scanners))
    }

    let (input, scanners) = scanners(input).unwrap();
    assert_eq!(input, "");
    scanners
}

fn orientations() -> Vec<Orientation> {
    let mut orientations = vec![];
    for p in vec![0usize, 1usize, 2usize].iter().permutations(3) {
        for dx in [1, -1] {
            for dy in [1, -1] {
                for dz in [1, -1] {
                    orientations.push((dx, dy, dz, *p[0], *p[1], *p[2]));
                }
            }
        }
    }
    orientations
}

fn change_orientation(pos: Position, (dx, dy, dz, p1, p2, p3): Orientation) -> Position {
    let new_pos = [pos.0 * dx, pos.1 * dy, pos.2 * dz];
    (new_pos[p1], new_pos[p2], new_pos[p3])
}

fn distance(a: &Position, b: &Position) -> Position {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn add(a: &Position, b: &Position) -> Position {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn manhattan_distance(a: &Position, b: &Position) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1) + i32::abs(a.2 - b.2)
}

fn solve(input: &str) -> (usize, usize) {
    let scanners = parse(input);
    let mut known_scanners = HashMap::new();
    known_scanners.insert(0, scanners[0].1.clone());
    let mut scanner_positions: Vec<Position> = vec![(0, 0, 0)];
    let orientations = orientations();
    while known_scanners.len() < scanners.len() {
        for (_, base_positions) in known_scanners.clone().iter() {
            for (num, positions) in scanners.iter() {
                if !known_scanners.contains_key(&num) {
                    for orientation in orientations.clone() {
                        let pos: Vec<Position> = positions
                            .iter()
                            .map(|p| change_orientation(p.clone(), orientation))
                            .collect();

                        let mut distances = HashMap::new();
                        for a in base_positions.iter() {
                            for b in pos.iter() {
                                let dist = distance(a, b);
                                *distances.entry(dist).or_insert(0) += 1;
                            }
                        }

                        if let Some(dist) =
                            distances.iter().find(|(_, v)| **v >= 12).map(|(k, _)| k)
                        {
                            scanner_positions.push(dist.clone());
                            let adjusted = pos.iter().map(|p| add(p, dist)).collect();
                            known_scanners.insert(*num, adjusted);
                        }
                    }
                }
            }
        }
    }

    let unique_beacons: HashSet<Position> = known_scanners
        .values()
        .flat_map(|v| v.iter().map(|b| b.clone()))
        .collect();

    let max_dist = scanner_positions
        .iter()
        .permutations(2)
        .map(|perm| manhattan_distance(perm[0], perm[1]))
        .max()
        .unwrap() as usize;

    (unique_beacons.len(), max_dist)
}

fn main() {
    let (p1, p2) = solve(include_str!("in.txt"));
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[test]
fn test_part1() {
    assert_eq!(solve(include_str!("test.txt")).0, 79);
}

#[test]
fn test_part2() {
    assert_eq!(solve(include_str!("test.txt")).1, 3621);
}
