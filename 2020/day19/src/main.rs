use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Char(String),
    Seq(Vec<usize>),
    Or(Vec<Vec<usize>>),
}

fn parse_seq(s: &str) -> Vec<usize> {
    s.trim()
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_rule(s: &str) -> (usize, Rule) {
    let parts = s.split(':').collect::<Vec<&str>>();
    let id = parts[0].parse::<usize>().unwrap();

    let r = parts[1].trim();
    if r.starts_with('"') {
        (
            id,
            Rule::Char(String::from(
                *r.chars().collect::<Vec<char>>().get(1).unwrap(),
            )),
        )
    } else {
        let seqs = r.split('|').map(parse_seq).collect::<Vec<Vec<usize>>>();
        if seqs.len() == 1 {
            let n = seqs.get(0).unwrap();
            (id, Rule::Seq(n.clone()))
        } else {
            (id, Rule::Or(seqs))
        }
    }
}

fn parse_input(s: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let parts = s.split("\n\n").collect::<Vec<&str>>();
    let rules_str = &parts[0];
    let messages = &parts[1]
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();
    let rules = {
        let mut rules = HashMap::new();
        for (id, rule) in rules_str.split('\n').map(parse_rule) {
            rules.insert(id, rule);
        }
        rules
    };

    (rules, messages.clone())
}

fn resolve_rule(rule: &Rule, rules: &HashMap<usize, Rule>) -> Vec<String> {
    match rule {
        Rule::Char(c) => vec![c.clone()],
        Rule::Seq(c) => {
            let sub = c
                .iter()
                .map(|s| rules.get(s).unwrap())
                .map(|rule| resolve_rule(rule, rules))
                .collect::<Vec<_>>();
            let product = (&sub)
                .into_iter()
                .multi_cartesian_product()
                .unique()
                .map(|l| l.iter().map(|s| s.clone()).join(""))
                .collect::<Vec<_>>();
            product
        }
        Rule::Or(seqs) => seqs
            .iter()
            .flat_map(|sub| resolve_rule(&Rule::Seq(sub.clone()), rules))
            .unique()
            .collect::<Vec<_>>(),
    }
}

fn part1(input: &str) -> usize {
    let (rules, messages) = parse_input(input);
    // this is slow, but could be switched to matches_rule approach from part2, which is lot more efficient
    let resolved = resolve_rule(rules.get(&0).unwrap(), &rules);
    let possible_messages: HashSet<&String> = HashSet::from_iter(resolved.iter());
    messages
        .iter()
        .filter(|msg| possible_messages.contains(msg))
        .count()
}

fn matches_rule(msg: &str, rule: &Rule, rules: &HashMap<usize, Rule>) -> bool {
    fn sub_match(m: &str, rule: &Rule, rules: &HashMap<usize, Rule>) -> Vec<usize> {
        match rule {
            Rule::Char(c) => {
                if m.starts_with(c) {
                    vec![1]
                } else {
                    vec![]
                }
            }
            Rule::Or(seqs) => {
                let mut matches = vec![];
                for seq in seqs.iter() {
                    let mut start = vec![0];
                    for rule in seq.iter() {
                        let mut next = vec![];
                        for i in start.iter() {
                            for m in sub_match(&m[*i..], rules.get(rule).unwrap(), rules) {
                                next.push(m + i);
                            }
                        }
                        start = next;
                    }
                    for l in start {
                        matches.push(l);
                    }
                }
                matches
            }
            Rule::Seq(x) => sub_match(m, &Rule::Or(vec![x.clone()]), rules),
        }
    }

    sub_match(msg, rule, rules).iter().any(|n| *n == msg.len())
}

fn part2(input: &str) -> usize {
    let (mut rules, messages) = parse_input(input);
    rules.insert(8, Rule::Or(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Or(vec![vec![42, 31], vec![42, 11, 31]]));
    let rule = rules.get(&0).unwrap();
    messages
        .iter()
        .filter(|msg| matches_rule(msg, rule, &rules))
        .count()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 2);
    assert_eq!(part1(include_str!("test2.txt")), 3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test2.txt")), 12);
}
