use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let parts = line.split(" | ").collect::<Vec<_>>();
            let digits = parts[1].split(' ').collect::<Vec<_>>();
            digits
                .iter()
                .map(|digit| match digit.len() {
                    2 => 1,
                    3 => 1,
                    4 => 1,
                    7 => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn decode(
    signal: &str,
    sig_seg_map: &HashMap<char, char>,
    digit_map: &HashMap<String, usize>,
) -> Result<usize, ()> {
    let trans = signal
        .chars()
        .map(|c| *sig_seg_map.get(&c).unwrap())
        .sorted()
        .collect::<String>();
    if let Some(&digit) = digit_map.get(&trans) {
        Ok(digit)
    } else {
        Err(())
    }
}

fn decode_signals(input: &str, digit_map: &HashMap<String, usize>, chars: &Vec<char>) -> usize {
    let parts = input.split(" | ").collect::<Vec<_>>();
    let signals = parts[0].split(' ').collect::<Vec<_>>();
    let digits = parts[1].split(' ').collect::<Vec<_>>();
    let trans = chars
        .iter()
        .permutations(7)
        .par_bridge()
        .find_map_any(move |perm| {
            let trans = perm
                .iter()
                .enumerate()
                .map(|(i, &c)| (*c, chars[i]))
                .collect::<HashMap<char, char>>();
            if signals
                .iter()
                .all(|&signal| decode(signal, &trans, &digit_map).is_ok())
            {
                return Some(trans.clone());
            }
            None
        })
        .unwrap();
    digits
        .iter()
        .map(|d| decode(d, &trans, &digit_map).unwrap())
        .map(|d| format!("{}", d))
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let digit_segments = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    let digit_map = digit_segments
        .iter()
        .map(|s| s.chars().sorted().collect::<String>())
        .enumerate()
        .map(|(i, s)| (String::from(s.clone()), i))
        .collect::<HashMap<String, usize>>();
    let chars = "abcdefg".chars().collect::<Vec<_>>();
    input
        .split('\n')
        .map(|line| decode_signals(line, &digit_map, &chars))
        .sum()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("in.txt")));
    println!("Part 2: {:?}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 26);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 61229);
}
