use itertools::Itertools;
use std::collections::HashMap;

type Rules = HashMap<(char, char), char>;

fn parse_rules(rules_str: &str) -> Rules {
    rules_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            let mut ac = a.chars();
            (
                (ac.next().unwrap(), ac.next().unwrap()),
                b.chars().next().unwrap(),
            )
        })
        .collect()
}

fn step(s: String, rules: &Rules) -> String {
    let mut res = vec![s.chars().next().unwrap()];
    for (a, b) in s.chars().tuple_windows() {
        let r = rules.get(&(a, b)).unwrap();
        res.push(*r);
        res.push(b);
    }
    res.iter().collect()
}

fn count_chars(s: String) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for c in s.chars() {
        *result.entry(c).or_insert(0) += 1;
    }
    result
}

fn part1(input: &str) -> usize {
    let (start, rules_str) = input.split_once("\n\n").unwrap();
    let rules: Rules = parse_rules(rules_str);
    let mut cur = start.to_string();
    for _ in 0..10 {
        cur = step(cur, &rules);
    }
    let counts = count_chars(cur);
    let (min, max) = counts.values().minmax().into_option().unwrap();
    max - min
}

fn step2(count_map: &HashMap<(char, char), usize>, rules: &Rules) -> HashMap<(char, char), usize> {
    let mut next_count = HashMap::new();
    for (&(a, b), cur_count) in count_map {
        let r = rules.get(&(a, b)).unwrap();
        *next_count.entry((a, *r)).or_insert(0) += cur_count;
        *next_count.entry((*r, b)).or_insert(0) += cur_count;
    }
    next_count
}

fn part2(input: &str) -> usize {
    let (start, rules_str) = input.split_once("\n\n").unwrap();
    let rules: Rules = parse_rules(rules_str);

    let mut pair_count: HashMap<(char, char), usize> = HashMap::new();
    for pair in start.chars().tuple_windows::<(_, _)>() {
        *pair_count.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..40 {
        pair_count = step2(&pair_count, &rules);
    }

    let mut char_counts = HashMap::new();
    for ((a, _), count) in pair_count {
        *char_counts.entry(a).or_insert(0) += count;
    }
    *char_counts
        .entry(start.chars().last().unwrap())
        .or_insert(0) += 1;

    let (min, max) = char_counts.values().minmax().into_option().unwrap();
    max - min
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 1588);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 2188189693529);
}
