// Problem: https://adventofcode.com/2018/day/4

use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct SleepCycle {
    guard: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Event {
    BeginsShift(usize),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct Entry {
    minute: usize,
    event: Event,
}

impl Entry {
    fn parse(line: &str) -> Self {
        Entry {
            minute: line[15..17].parse().unwrap(),
            event: match &line[19..] {
                "falls asleep" => Event::FallsAsleep,
                "wakes up" => Event::WakesUp,
                _ => {
                    Event::BeginsShift(sscanf!(&line[19..], "Guard #{usize} begins shift").unwrap())
                }
            },
        }
    }
}

fn parse_sleep_cycles(entries: &[Entry]) -> Vec<SleepCycle> {
    let mut cycles: Vec<SleepCycle> = Vec::new();
    let mut cur_guard = None;
    let mut start = 0;
    for Entry { minute, event } in entries.iter() {
        match event {
            Event::BeginsShift(guard) => {
                cur_guard = Some(*guard);
            }
            Event::FallsAsleep => {
                start = *minute;
            }
            Event::WakesUp => {
                cycles.push(SleepCycle {
                    guard: cur_guard.unwrap(),
                    start,
                    end: *minute,
                });
            }
        }
    }
    cycles
}

fn parse_input(input: &str) -> Vec<SleepCycle> {
    let entries: Vec<_> = input.lines().sorted().map(Entry::parse).collect();
    parse_sleep_cycles(&entries)
}

fn part1(cycles: &[SleepCycle]) -> usize {
    let mut guards: HashMap<usize, usize> = HashMap::new();
    for SleepCycle { guard, start, end } in cycles.iter() {
        *guards.entry(*guard).or_default() += end - start;
    }
    let (best_guard, _) = guards.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();

    let mut minutes: HashMap<usize, usize> = HashMap::new();
    for SleepCycle { guard, start, end } in cycles.iter() {
        if guard == best_guard {
            for m in *start..*end {
                *minutes.entry(m).or_default() += 1;
            }
        }
    }
    minutes
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(best_minute, _)| best_guard * best_minute)
        .unwrap()
}

fn best_minute_for_guard(the_guard: usize, cycles: &[SleepCycle]) -> Option<(usize, usize, usize)> {
    let mut minutes: HashMap<usize, usize> = HashMap::new();
    for SleepCycle { guard, start, end } in cycles.iter() {
        if *guard == the_guard {
            for m in *start..*end {
                *minutes.entry(m).or_default() += 1;
            }
        }
    }
    if let Some((best_minute, amount)) = minutes.iter().max_by(|a, b| a.1.cmp(b.1)) {
        Some((the_guard, *best_minute, *amount))
    } else {
        None
    }
}

fn part2(cycles: &[SleepCycle]) -> usize {
    let mut guards = HashSet::new();
    for SleepCycle { guard, .. } in cycles.iter() {
        guards.insert(*guard);
    }
    guards
        .iter()
        .filter_map(|g| best_minute_for_guard(*g, cycles))
        .max_by(|&(_, _, a), &(_, _, b)| a.cmp(&b))
        .map(|(best_guard, best_minute, _)| best_guard * best_minute)
        .unwrap()
}

fn main() {
    let cycles = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&cycles));
    println!("Part 2: {:?}", part2(&cycles));
}

#[test]
fn test_part1() {
    assert_eq!(part1(&parse_input(include_str!("test.txt"))), 240);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse_input(include_str!("test.txt"))), 4455);
}
