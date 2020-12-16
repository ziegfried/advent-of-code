#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
struct Field {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl Field {
    pub fn from_str(s: &str) -> Self {
        lazy_static! {
            static ref FIELD_RE: Regex = Regex::new(r"(.+?): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        }
        let m = FIELD_RE.captures(s).unwrap();
        Field {
            name: String::from(&m[1]),
            range1: (*&m[2].parse::<usize>().unwrap())..=(*&m[3].parse::<usize>().unwrap()),
            range2: (*&m[4].parse::<usize>().unwrap())..=(*&m[5].parse::<usize>().unwrap()),
        }
    }

    pub fn is_value_possible(&self, val: usize) -> bool {
        self.range1.contains(&val) || self.range2.contains(&val)
    }
}

fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(',')
        .map(|p| p.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_input(s: &str) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let parts = s.split("\n\n").collect::<Vec<_>>();
    let fields = parts[0]
        .split('\n')
        .map(Field::from_str)
        .collect::<Vec<_>>();
    let my_ticket = parse_numbers(parts[1].split('\n').collect::<Vec<_>>().get(1).unwrap());
    let other_tickets = parts[2]
        .split('\n')
        .skip(1)
        .map(parse_numbers)
        .collect::<Vec<Vec<usize>>>();
    (fields, my_ticket, other_tickets)
}

fn part1(s: &str) -> usize {
    let (fields, _my_ticket, other_tickets) = parse_input(s);
    other_tickets
        .iter()
        .map(|t| {
            t.iter()
                .map(|no| {
                    if fields.iter().any(|f| f.is_value_possible(*no)) {
                        0
                    } else {
                        *no
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn part2(s: &str) -> usize {
    let (fields, my_ticket, other_tickets) = parse_input(s);
    let valid_tickets = other_tickets
        .iter()
        .filter(|t| {
            t.iter()
                .all(|no| fields.iter().any(|f| f.is_value_possible(*no)))
        })
        .rev()
        .collect::<Vec<&Vec<usize>>>();
    let mut possible_tickets: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut field_map: HashMap<usize, String> = HashMap::new();
    for idx in 0..(fields.len()) {
        let candidates = fields
            .iter()
            .filter(|f| {
                valid_tickets
                    .iter()
                    .all(|t| f.is_value_possible(*t.get(idx).unwrap()))
            })
            .map(|f| f.name.clone());
        possible_tickets.insert(idx, HashSet::from_iter(candidates));
    }

    loop {
        let remaining = possible_tickets.clone();
        let found = remaining
            .iter()
            .filter(|(_, fields)| fields.len() == 1)
            .collect::<Vec<_>>();
        if found.len() == 0 {
            break;
        }
        for (idx, fields) in found.iter() {
            let field = fields.iter().next().unwrap().clone();
            field_map.insert(**idx, field.clone());
            for val in possible_tickets.values_mut() {
                val.remove(&field);
            }
        }
    }
    field_map
        .iter()
        .filter(|(_, field_name)| field_name.starts_with("departure"))
        .map(|(v, _)| my_ticket.get(*v).unwrap())
        .fold(1, |a, b| a * b)
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("in.txt")));
    println!("Part 2: {:?}", part2(include_str!("in.txt")));
}

#[test]
fn test_parse_field() {
    let field = Field::from_str("class: 1-3 or 5-7");
    assert_eq!(
        field,
        Field {
            name: String::from("class"),
            range1: 1..=3,
            range2: 5..=7,
        }
    );

    assert_eq!(field.range1.contains(&3), true);
    assert_eq!(field.range1.contains(&5), false);
}
