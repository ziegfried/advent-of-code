// Problem: https://adventofcode.com/2018/day/1
use std::collections::HashSet;

fn part1(input: &str) -> i32 {
    input.lines().map(|l| l.parse::<i32>().unwrap()).sum()
}

fn part2(input: &str) -> i32 {
    let numbers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let mut freq: i32 = 0;
    let mut seen = HashSet::new();

    for n in numbers.iter().cloned().cycle() {
        if seen.contains(&freq) {
            return freq;
        }
        seen.insert(freq);
        freq += n;
    }

    unreachable!()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 4);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 10);
}
