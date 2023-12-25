// Problem: https://adventofcode.com/2023/day/24

use geo::Line;
use itertools::Itertools;
use std::ops::{Add, Mul};
use z3::{ast::Ast, SatResult};

type Result = usize;

type Input = Vec<((i64, i64, i64), (i64, i64, i64))>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let (x, y, z) = (
                parts
                    .next()
                    .unwrap()
                    .strip_suffix(',')
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
                parts
                    .next()
                    .unwrap()
                    .strip_suffix(',')
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            );
            assert_eq!(parts.next().unwrap(), "@");
            let (vx, vy, vz) = (
                parts
                    .next()
                    .unwrap()
                    .strip_suffix(',')
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
                parts
                    .next()
                    .unwrap()
                    .strip_suffix(',')
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            );
            ((x, y, z), (vx, vy, vz))
        })
        .collect()
}

// ------------------------------------------

fn to_2d_line(((x, y, _), (vx, vy, _)): &((i64, i64, i64), (i64, i64, i64))) -> Line {
    Line {
        start: (*x as f64, *y as f64).into(),
        end: (
            (*x as f64 + *vx as f64 * 400000000000000.0),
            (*y as f64 + *vy as f64 * 400000000000000.0),
        )
            .into(),
    }
}

fn part1(input: &Input, min: f64, max: f64) -> Result {
    let lines: Vec<Line> = input.iter().map(to_2d_line).collect();
    let range = min..=max;
    let mut count = 0;
    for (a, b) in lines.iter().tuple_combinations() {
        if let Some(intersection) = intersect2d::intersect(a, b) {
            match intersection {
                intersect2d::Intersection::Intersection(coord) => {
                    if range.contains(&coord.x) && range.contains(&coord.y) {
                        count += 1;
                    }
                }
                intersect2d::Intersection::OverLap(_) => panic!(),
            }
        }
    }
    count
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input, 7.0, 27.0), 2);
}

// ------------------------------------------

fn z3_solve(solver: &z3::Solver, ctx: &z3::Context, input: &Input) -> i64 {
    let s_x = z3::ast::BV::new_const(ctx, "x", 64);
    let s_y = z3::ast::BV::new_const(ctx, "y", 64);
    let s_z = z3::ast::BV::new_const(ctx, "z", 64);
    let s_vx = z3::ast::BV::new_const(ctx, "vx", 64);
    let s_vy = z3::ast::BV::new_const(ctx, "vy", 64);
    let s_vz = z3::ast::BV::new_const(ctx, "vz", 64);
    let zero = z3::ast::BV::from_i64(ctx, 0, 64);
    for (i, ((x, y, z), (vx, vy, vz))) in input.iter().enumerate() {
        let t = z3::ast::BV::new_const(ctx, format!("t_{}", i), 64);
        solver.assert(&t.bvuge(&zero));

        let x = z3::ast::BV::from_i64(ctx, *x, 64);
        let y = z3::ast::BV::from_i64(ctx, *y, 64);
        let z = z3::ast::BV::from_i64(ctx, *z, 64);
        let vx = z3::ast::BV::from_i64(ctx, *vx, 64);
        let vy = z3::ast::BV::from_i64(ctx, *vy, 64);
        let vz = z3::ast::BV::from_i64(ctx, *vz, 64);
        solver.assert(&(&s_x).add((&s_vx).mul(&t))._eq(&x.add(vx.mul(&t))));
        solver.assert(&(&s_y).add((&s_vy).mul(&t))._eq(&y.add(vy.mul(&t))));
        solver.assert(&(&s_z).add((&s_vz).mul(&t))._eq(&z.add(vz.mul(&t))));
    }
    assert_eq!(solver.check(), SatResult::Sat);
    let model = solver.get_model().unwrap();
    let res = (&s_x).add((&s_y).add(&s_z));
    model.eval(&res, false).unwrap().as_i64().unwrap()
}

fn part2(input: &Input) -> i64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    z3_solve(&solver, &ctx, input)
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 47);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!(
        "Part 1: {:?}",
        part1(&input, 200000000000000f64, 400000000000000f64)
    );
    println!("Part 2: {:?}", part2(&input));
}
