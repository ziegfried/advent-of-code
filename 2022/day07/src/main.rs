// Problem: https://adventofcode.com/2022/day/7

use std::collections::HashMap;

type Result = usize;

fn compute_totals_used(input: &str) -> HashMap<Vec<String>, usize> {
    let mut dirs: HashMap<Vec<String>, Vec<usize>> = HashMap::new();
    let mut pwd: Vec<String> = vec![];
    for line in input.lines() {
        if line == "$ ls" || line.starts_with("dir ") {
            // ignore
        } else if line.starts_with("$ cd") {
            let target = line.split_whitespace().nth(2).unwrap();
            match target {
                "/" => pwd = vec![],
                "." => {}
                ".." => {
                    pwd.pop();
                }
                _ => {
                    pwd.push(target.to_string());
                }
            }

            if !dirs.contains_key(&pwd) {
                dirs.insert(pwd.clone(), vec![]);
            }
        } else {
            let mut parts = line.split_whitespace();
            let size = parts.next().unwrap().parse::<usize>().unwrap();
            dirs.entry(pwd.clone()).or_default().push(size);
        }
    }
    let mut totals = HashMap::new();
    let mut keys = dirs.keys().cloned().collect::<Vec<Vec<String>>>();
    keys.sort_by_key(|b| std::cmp::Reverse(b.len()));
    for dir in keys {
        let mut total = 0;
        let files = dirs.get(&dir).unwrap();
        for file in files {
            total += file;
        }
        for sub in dirs.keys() {
            if sub.len() == dir.len() + 1 && sub.starts_with(dir.as_slice()) {
                let child_total = totals.get(sub).unwrap();
                total += child_total;
            }
        }
        totals.insert(dir, total);
    }
    totals
}

fn part1(input: &str) -> Result {
    let totals = compute_totals_used(input);
    totals.values().filter(|&&v| v <= 100000).sum()
}

fn part2(input: &str) -> Result {
    let available = 70000000;
    let need = 30000000;
    let totals = compute_totals_used(input);
    let unused = available - totals.get(&vec![]).unwrap();
    let min_amount_to_free = need - unused;

    *totals
        .values()
        .filter(|&&v| v >= min_amount_to_free)
        .min()
        .unwrap()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 95437);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 24933642);
}
