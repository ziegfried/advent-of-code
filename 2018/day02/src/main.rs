// Problem: https://adventofcode.com/2018/day/2

use std::collections::HashMap;
use std::iter::zip;

fn has_letter_with_count(word: &str, count: usize) -> bool {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for ch in word.chars() {
        *counts.entry(ch).or_default() += 1;
    }
    counts.values().any(|v| *v == count)
}

fn part1(input: &str) -> usize {
    let words: Vec<&str> = input.lines().collect();
    let twos = words.iter().filter(|w| has_letter_with_count(w, 2)).count();
    let threes = words.iter().filter(|w| has_letter_with_count(w, 3)).count();

    twos * threes
}

fn part2(input: &str) -> String {
    let words: Vec<&str> = input.lines().collect();
    let l = words[0].len();
    for a in words.iter() {
        for b in words.iter() {
            let same: Vec<(char, char)> =
                zip(a.chars(), b.chars()).filter(|(a, b)| a == b).collect();
            if same.len() == l - 1 {
                return same.iter().map(|(l, _)| l).collect();
            }
        }
    }
    unreachable!()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 12);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test2.txt")), "fgij".to_string());
}
