// Problem: https://adventofcode.com/2022/day/19

use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    ore_robot_cost_ore: usize,
    clay_robot_cost_ore: usize,
    obsidian_robot_cost_ore: usize,
    obsidian_robot_cost_clay: usize,
    geode_robot_cost_ore: usize,
    geode_robot_cost_obsidian: usize,
}

fn parse_blueprint(line: &str) -> Blueprint {
    let (
        id,
        ore_robot_cost_ore,
        clay_robot_cost_ore,
        obsidian_robot_cost_ore,
        obsidian_robot_cost_clay,
        geode_robot_cost_ore,
        geode_robot_cost_obsidian,
    ) = sscanf::sscanf!(
        line,
        "Blueprint {usize}: Each ore robot costs {usize} ore. Each clay robot costs {usize} ore. Each obsidian robot costs {usize} ore and {usize} clay. Each geode robot costs {usize} ore and {usize} obsidian."
    )
    .unwrap();

    Blueprint {
        id,
        ore_robot_cost_ore,
        clay_robot_cost_ore,
        obsidian_robot_cost_ore,
        obsidian_robot_cost_clay,
        geode_robot_cost_ore,
        geode_robot_cost_obsidian,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    minute: usize,

    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,

    ore_collectors: usize,
    clay_collectors: usize,
    obidian_collectors: usize,
    geode_collectors: usize,
}

fn can_make_geode_collector_every_minute(state: &State, blueprint: &Blueprint) -> bool {
    state.obidian_collectors >= blueprint.geode_robot_cost_obsidian
        && state.ore_collectors >= blueprint.geode_robot_cost_ore
}

fn run_robots(state: &mut State) {
    state.ore += state.ore_collectors;
    state.clay += state.clay_collectors;
    state.obsidian += state.obidian_collectors;
    state.geode += state.geode_collectors;
    state.minute += 1;
}

fn can_eleminate_early(state: &State, max: usize, time: usize) -> bool {
    let mut making = state.geode_collectors;
    let mut geodes = state.geode;
    for _ in state.minute..=time {
        geodes += making;
        making += 1;
    }
    geodes < max
}

fn max_obsidian_output(blueprint: &Blueprint, time: usize) -> usize {
    let max_ore_prod = *vec![
        blueprint.ore_robot_cost_ore,
        blueprint.clay_robot_cost_ore,
        blueprint.obsidian_robot_cost_ore,
        blueprint.geode_robot_cost_ore,
    ]
    .iter()
    .max()
    .unwrap();
    let max_clay_prod = blueprint.obsidian_robot_cost_clay;
    let max_obsidian_prod = blueprint.geode_robot_cost_obsidian;
    let mut queue: VecDeque<State> = vec![State {
        minute: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_collectors: 1,
        clay_collectors: 0,
        obidian_collectors: 0,
        geode_collectors: 0,
    }]
    .into();
    let mut seen = HashSet::new();
    let mut max = 0;

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();

        if state.minute == time {
            if state.geode > max {
                max = state.geode;
            }
            continue;
        }

        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);

        if can_eleminate_early(&state, max, time) {
            continue;
        }

        if can_make_geode_collector_every_minute(&state, blueprint) {
            let mut making = state.geode_collectors;
            let mut geodes = state.geode;

            for _ in state.minute..time {
                geodes += making;
                making += 1;
            }
            if geodes > max {
                max = geodes;
            }
            continue;
        }

        if state.obsidian >= blueprint.geode_robot_cost_obsidian
            && state.ore >= blueprint.geode_robot_cost_ore
        {
            let mut state = state;
            state.ore -= blueprint.geode_robot_cost_ore;
            state.obsidian -= blueprint.geode_robot_cost_obsidian;
            run_robots(&mut state);
            state.geode_collectors += 1;
            queue.push_back(state);
        }
        if state.obidian_collectors < max_obsidian_prod
            && state.clay >= blueprint.obsidian_robot_cost_clay
            && state.ore >= blueprint.obsidian_robot_cost_ore
        {
            let mut state = state;
            state.ore -= blueprint.obsidian_robot_cost_ore;
            state.clay -= blueprint.obsidian_robot_cost_clay;
            run_robots(&mut state);
            state.obidian_collectors += 1;
            queue.push_back(state);
        }
        if state.clay_collectors < max_clay_prod && state.ore >= blueprint.clay_robot_cost_ore {
            let mut state = state;
            state.ore -= blueprint.clay_robot_cost_ore;
            run_robots(&mut state);
            state.clay_collectors += 1;
            queue.push_back(state);
        }
        if state.ore_collectors < max_ore_prod && state.ore >= blueprint.ore_robot_cost_ore {
            let mut state = state;
            state.ore -= blueprint.ore_robot_cost_ore;
            run_robots(&mut state);
            state.ore_collectors += 1;
            queue.push_back(state);
        }

        let mut state = state;
        run_robots(&mut state);
        queue.push_back(state);
    }
    max
}

fn part1(input: &str) -> usize {
    let blueprints: Vec<_> = input.trim().lines().map(parse_blueprint).collect();
    blueprints
        .par_iter()
        .map(|b| (b, max_obsidian_output(b, 24)))
        .map(|(b, mx)| b.id * mx)
        .sum()
}

fn part2(input: &str) -> usize {
    let blueprints: Vec<_> = input.trim().lines().map(parse_blueprint).collect();
    blueprints[0..3]
        .par_iter()
        .map(|b| max_obsidian_output(b, 32))
        .product()
}

fn main() {
    vec![
        (1, part1 as fn(&str) -> usize),
        (2, part2 as fn(&str) -> usize),
    ]
    .par_iter()
    .map(|(part, solve)| (part, solve(include_str!("input.txt"))))
    .for_each(|(part, result)| {
        println!("Part {}: {}", part, result);
    });
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 33);
}

#[test]
fn test_part2() {
    let lines: Vec<&str> = include_str!("test.txt").trim().lines().collect();
    assert_eq!(max_obsidian_output(&parse_blueprint(lines[0]), 32), 56);
    assert_eq!(max_obsidian_output(&parse_blueprint(lines[1]), 32), 62);
}
