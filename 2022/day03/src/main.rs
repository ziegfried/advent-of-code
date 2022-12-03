use itertools::Itertools;
use std::collections::HashSet;

fn common_item(rucksack: &str) -> char {
    let first: HashSet<char> = rucksack[0..rucksack.len() / 2].chars().collect();
    for c in rucksack[rucksack.len() / 2..].chars() {
        if first.contains(&c) {
            return c;
        }
    }
    panic!("no common item found");
}

fn char_priority(c: char) -> usize {
    if ('a'..='z').contains(&c) {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

fn part1(input: &str) -> usize {
    input.lines().map(common_item).map(char_priority).sum()
}

fn badge_item((a, b, c): (&str, &str, &str)) -> char {
    let a = a.chars().collect::<HashSet<_>>();
    let b = b.chars().collect::<HashSet<_>>();
    for c in c.chars() {
        if a.contains(&c) && b.contains(&c) {
            return c;
        }
    }
    panic!("no badge item found");
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(a, b, c)| badge_item((a, b, c)))
        .map(char_priority)
        .sum()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("in.txt")));
    println!("Part 2: {:?}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 157);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 70);
}
