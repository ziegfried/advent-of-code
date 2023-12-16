// Problem: https://adventofcode.com/2023/day/5

use itertools::Itertools;
use rayon::prelude::*;
use sscanf::sscanf;

type Int = i64;

type Result = Int;

type Input = (Vec<Int>, Vec<(String, String, Vec<(Int, Int, Int)>)>);

fn parse_input(input: &str) -> Input {
    let (header, body) = input.trim().split_once("\n\n").unwrap();

    let seeds: Vec<Int> = header
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse::<Int>().unwrap())
        .collect();

    let body = body
        .split("\n\n")
        .map(|chunk| {
            let (header, lines) = chunk.split_once('\n').unwrap();
            let (from, to) = sscanf!(header, "{String}-to-{String} map:").unwrap();
            let map = lines
                .split('\n')
                .map(|line| sscanf!(line, "{i64} {i64} {i64}").unwrap())
                .sorted()
                .collect::<Vec<(Int, Int, Int)>>();

            (from, to, map)
        })
        .collect();

    (seeds, body)
}

#[derive(Debug, Clone)]
struct OffsetRanges(Vec<(Int, Int, Int)>);

impl OffsetRanges {
    fn from_map(map: &[(Int, Int, Int)]) -> Self {
        Self(
            map.iter()
                .cloned()
                .map(|(dest, source, count)| (source, source + count, dest - source))
                .collect(),
        )
    }
    fn offset_at(&self, n: Int) -> Int {
        for (start, end, offset) in self.0.iter() {
            if &n >= start && &n < end {
                return *offset;
            }
        }
        0
    }
}

// ------------------------------------------

fn resolve_location(
    cur_type: &String,
    val: Int,
    maps: &Vec<(String, String, OffsetRanges)>,
) -> Int {
    let (_, next_type, ranges) = maps.iter().find(|(from, _, _)| from == cur_type).unwrap();
    let next_offset = ranges.offset_at(val);
    let next_val = val + next_offset;
    if next_type == "location" {
        next_val
    } else {
        resolve_location(next_type, next_val, maps)
    }
}

fn part1((seeds, maps): &Input) -> Result {
    let ranges = maps
        .iter()
        .map(|(from, to, map)| (from.clone(), to.clone(), OffsetRanges::from_map(map)))
        .collect::<Vec<_>>();
    seeds
        .iter()
        .map(|seed| resolve_location(&"seed".to_string(), *seed, &ranges))
        .min()
        .unwrap()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 35);
    let input = parse_input(include_str!("input.txt"));
    assert_eq!(part1(&input), 600279879);
}

// ------------------------------------------

#[inline]
fn resolve_fast(seed: Int, ranges: &Vec<OffsetRanges>) -> Int {
    let mut cur = seed;
    for r in ranges {
        cur = cur + r.offset_at(cur);
    }
    cur
}

fn part2((seeds, maps): &Input) -> Result {
    let ranges = maps
        .iter()
        .map(|(from, to, map)| (from.clone(), to.clone(), OffsetRanges::from_map(map)))
        .collect::<Vec<_>>();
    let range_list: Vec<OffsetRanges> = ranges.iter().cloned().map(|(_, _, list)| list).collect();
    seeds
        .iter()
        .cloned()
        .tuples()
        .collect::<Vec<(Int, Int)>>()
        .par_iter()
        .flat_map(|(a, b)| *a..(a + b))
        .map(|seed| resolve_fast(seed, &range_list))
        .min()
        .unwrap()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 46);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
