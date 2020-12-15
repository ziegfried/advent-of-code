#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate regex;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"^(.+?) would (gain|lose) (\d+) happiness units by sitting next to (.+?)\.$")
            .unwrap();
}

fn parse_line(s: &str) -> (String, String, isize) {
    let m = LINE_RE.captures(s).unwrap();
    return (
        String::from(&m[1]),
        String::from(&m[4]),
        &m[3].parse::<isize>().unwrap() * if &m[2] == "gain" { 1 } else { -1 },
    );
}

fn calc_max_happiness_gains(
    names: HashSet<String>,
    map: HashMap<(String, String), isize>,
) -> isize {
    let name_count = &names.len();
    names
        .into_iter()
        .permutations(*name_count)
        .map(|arrangement| {
            let happiness_gains: isize = arrangement
                .windows(2)
                .map(|pair| {
                    map.get(&(pair[0].clone(), pair[1].clone())).unwrap()
                        + map.get(&(pair[1].clone(), pair[0].clone())).unwrap()
                })
                .sum();
            let first = arrangement.get(0).unwrap();
            let last = arrangement.get(arrangement.len() - 1).unwrap();
            let edge_happiness = map.get(&(first.clone(), last.clone())).unwrap()
                + map.get(&(last.clone(), first.clone())).unwrap();
            happiness_gains + edge_happiness
        })
        .max()
        .unwrap()
}

fn part1(input: &str) -> isize {
    let mut map = HashMap::new();
    let mut names = HashSet::new();
    for (a, b, gain) in input.split('\n').map(parse_line) {
        map.insert((a.clone(), b.clone()), gain);
        names.insert(a);
        names.insert(b);
    }
    calc_max_happiness_gains(names, map)
}

fn part2(input: &str) -> isize {
    let me = String::from("Me");
    let mut map = HashMap::new();
    let mut names = HashSet::new();
    names.insert(me.clone());
    for (a, b, gain) in input.split('\n').map(parse_line) {
        map.insert((a.clone(), b.clone()), gain);
        names.insert(a.clone());
        names.insert(b.clone());
        map.insert((a.clone(), me.clone()), 0);
        map.insert((me.clone(), a.clone()), 0);
        map.insert((b.clone(), me.clone()), 0);
        map.insert((me.clone(), b.clone()), 0);
    }
    calc_max_happiness_gains(names, map)
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("Alice would gain 54 happiness units by sitting next to Bob."),
        ("Alice".to_string(), "Bob".to_string(), 54)
    );
    assert_eq!(
        parse_line("Alice would lose 79 happiness units by sitting next to Carol."),
        ("Alice".to_string(), "Carol".to_string(), -79)
    );
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 330);
}
