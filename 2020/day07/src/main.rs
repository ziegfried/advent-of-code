#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct Rule {
    count: usize,
    color: String,
}

#[derive(Debug, PartialEq)]
struct Bag {
    color: String,
    rules: Vec<Rule>,
}

fn parse_rules(s: &str) -> Vec<Rule> {
    if s == "no other bags" {
        return vec![];
    }
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) (.+?) bags?").unwrap();
    }
    s.split(", ")
        .map(|s| {
            let m = RE.captures(s).unwrap();
            let color = String::from(&m[2]);
            let count = (&m[1]).parse::<usize>().unwrap();
            Rule {
                color: color,
                count: count,
            }
        })
        .collect::<Vec<Rule>>()
}

fn parse_line(s: &str) -> Bag {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+?) bags contain (.+)\.").unwrap();
    }
    if let Some(m) = RE.captures(s) {
        let color = String::from(&m[1]);
        let rules = parse_rules(&m[2]);
        Bag {
            color: color,
            rules: rules,
        }
    } else {
        panic!("no match")
    }
}

fn resolve_bag_count(color: String, bags: &Vec<Bag>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(cached) = cache.get(&color) {
        return *cached + 1;
    }
    let bag = bags.iter().find(|b| b.color == color).unwrap();
    let contents = bag
        .rules
        .iter()
        .map(|r| {
            let resolved = resolve_bag_count(r.color.clone(), &bags, &mut cache.clone());
            return r.count * resolved;
        })
        .sum();
    cache.insert(color, contents);
    return contents + 1;
}

fn main() {
    let bags = include_str!("../in.txt")
        .split('\n')
        .map(|s| parse_line(s))
        .collect::<Vec<_>>();

    let mut allowed_colors = HashSet::new();
    let start_color = String::from("shiny gold");
    allowed_colors.insert(&start_color);
    loop {
        let mut more = false;
        for b in bags.iter() {
            if !allowed_colors.contains(&b.color)
                && b.rules.iter().any(|r| allowed_colors.contains(&r.color))
            {
                allowed_colors.insert(&b.color);
                more = true;
            }
        }
        if !more {
            break;
        }
    }
    let part1 =allowed_colors.len() - 1;

    let mut cache: HashMap<String, usize> = HashMap::new();
    let part2 = resolve_bag_count(start_color, &bags, &mut cache) - 1;
    
    dbg!(part1, part2);
}
