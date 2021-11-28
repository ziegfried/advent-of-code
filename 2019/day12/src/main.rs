use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;
use std::{collections::HashSet, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Voxel {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Voxel {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Voxel {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Moon {
    pub id: usize,
    pub location: Voxel,
    pub velocity: Voxel,
}
impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Moon {}

fn voxel(x: i32, y: i32, z: i32) -> Voxel {
    Voxel { x: x, y: y, z: z }
}

fn parse_moon_locations(input: &str) -> Vec<Voxel> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
    }
    input
        .split('\n')
        .map(|line| {
            let captures = RE.captures(line).unwrap();

            voxel(
                captures["x"].parse::<i32>().unwrap(),
                captures["y"].parse::<i32>().unwrap(),
                captures["z"].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn gravity_impact(val1: i32, val2: i32) -> i32 {
    if val1 > val2 {
        -1
    } else if val1 < val2 {
        1
    } else {
        0
    }
}

fn apply_gravity(moon: &mut Moon, all_moons: &Vec<Moon>) {
    for other in all_moons.iter() {
        if other != moon {
            moon.velocity = moon.velocity
                + voxel(
                    gravity_impact(moon.location.x, other.location.x),
                    gravity_impact(moon.location.y, other.location.y),
                    gravity_impact(moon.location.z, other.location.z),
                );
        }
    }
}

fn apply_velocity(moon: &mut Moon) {
    moon.location = moon.location + moon.velocity;
}

fn energy(voxel: Voxel) -> usize {
    (voxel.x.abs() + voxel.y.abs() + voxel.z.abs()) as usize
}

fn voxel_to_string(voxel: Voxel) -> String {
    format!("<x={:3}, y={:3}, z={:3}>", voxel.x, voxel.y, voxel.z)
}

fn print_moons(moons: &Vec<Moon>) {
    for moon in moons.iter() {
        println!(
            "pos={}, vel={}",
            voxel_to_string(moon.location),
            voxel_to_string(moon.velocity)
        );
    }
}

const DEBUG: bool = false;

fn part1(input: &str, iterations: usize) -> usize {
    let mut moons = parse_moon_locations(input)
        .iter()
        .enumerate()
        .map(|(number, loc)| Moon {
            id: number,
            location: *loc,
            velocity: voxel(0, 0, 0),
        })
        .collect::<Vec<_>>();
    if DEBUG {
        print_moons(&moons);
        println!();
    }

    for step in 0..iterations {
        let all_moons = moons.clone();
        for moon in moons.iter_mut() {
            apply_gravity(moon, &all_moons);
        }
        for moon in moons.iter_mut() {
            apply_velocity(moon);
        }

        if DEBUG {
            println!("After step {}", step + 1);
            print_moons(&moons);
            println!();
        }
    }

    let mut total_energy = 0;
    for moon in moons.iter() {
        let pot = energy(moon.location);
        let kin = energy(moon.velocity);
        total_energy += pot * kin;
    }
    total_energy
}

fn find_rep(axis: Vec<(i32, i32)>) -> usize {
    let mut seen = HashSet::<Vec<(i32, i32)>>::new();
    let mut cur = axis;
    loop {
        seen.insert(cur.clone());
        for i in 0..cur.len() {
            for j in 0..cur.len() {
                if i != j {
                    cur[i].1 += gravity_impact(cur[i].0, cur[j].0);
                }
            }
        }
        for i in 0..cur.len() {
            cur[i].0 += cur[i].1;
        }
        if seen.contains(&cur) {
            return seen.len();
        }
    }
}

fn part2(input: &str) -> usize {
    let moons = parse_moon_locations(input)
        .iter()
        .enumerate()
        .map(|(number, loc)| Moon {
            id: number,
            location: *loc,
            velocity: voxel(0, 0, 0),
        })
        .collect::<Vec<_>>();

    let x_axis = moons
        .iter()
        .map(|moon| (moon.location.x, moon.velocity.x))
        .collect::<Vec<_>>();
    let y_axis = moons
        .iter()
        .map(|moon| (moon.location.y, moon.velocity.y))
        .collect::<Vec<_>>();
    let z_axis = moons
        .iter()
        .map(|moon| (moon.location.z, moon.velocity.z))
        .collect::<Vec<_>>();

    let x_reps = find_rep(x_axis);
    let y_reps = find_rep(y_axis);
    let z_reps = find_rep(z_axis);

    x_reps.lcm(&y_reps.lcm(&z_reps))
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt"), 1000));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test1.txt"), 10), 179);
    assert_eq!(part1(include_str!("test2.txt"), 100), 1940);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test1.txt")), 2772);
    assert_eq!(part2(include_str!("test2.txt")), 4686774924);
}
