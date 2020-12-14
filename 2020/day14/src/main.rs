extern crate itertools;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn parse_mask(s: &str) -> (u64, u64, u64) {
    let mut set_bits: u64 = 0;
    let mut unset_bits: u64 = 0;
    let mut floating_bits: u64 = 0;
    for (i, c) in s.chars().rev().enumerate() {
        let bit = 2u64.pow(i as u32);
        match c {
            'X' => {
                unset_bits |= bit;
                floating_bits |= bit;
            }
            '1' => {
                set_bits |= bit;
                unset_bits |= bit;
            }
            '0' => {
                // noop
            }
            _ => {
                panic!(format!("unexpected char {:?}", c));
            }
        }
    }
    (set_bits, unset_bits, floating_bits)
}

#[derive(Debug)]
enum Line {
    Mask((u64, u64, u64)),
    MemSet(u64, u64),
}

fn parse_input(s: &str) -> Vec<Line> {
    let mask_re = Regex::new(r"mask = ([X01]+)").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    s.split('\n')
        .map(|line| {
            if let Some(mm) = mask_re.captures(line) {
                let mask = parse_mask(&mm[1]);
                Line::Mask(mask)
            } else if let Some(m) = mem_re.captures(line) {
                Line::MemSet(
                    *(&m[1].parse::<u64>().unwrap()),
                    *(&m[2].parse::<u64>().unwrap()),
                )
            } else {
                panic!(format!("no match {:?}", line))
            }
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = (0, 0, 0);
    for item in parse_input(input) {
        match item {
            Line::Mask(m) => {
                mask = m;
            }
            Line::MemSet(addr, value) => {
                let (set_bits, unset_bits, _) = mask;
                let value = (value | set_bits) & unset_bits;
                mem.insert(addr, value);
            }
        }
    }
    mem.values().sum()
}

fn floating_combinations(mask: u64) -> Vec<u64> {
    let mut bits = 0;
    for i in 0..36 {
        if mask & 2u64.pow(i) > 0 {
            bits = bits * 2 + 1;
        }
    }
    (0..=bits)
        .map(|v| {
            let mut val: u64 = 0;
            let mut idx = 0;
            for i in 0..36 {
                let bit = 2u64.pow(i);
                if mask & bit > 0 {
                    if v & 2u64.pow(idx) > 0 {
                        val |= bit;
                    }
                    idx += 1;
                }
            }
            val
        })
        .collect()
}

fn part2(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = (0, 0, 0);
    for item in parse_input(input) {
        match item {
            Line::Mask(m) => {
                mask = m;
            }
            Line::MemSet(addr, value) => {
                let (set_bits, _, floating_bits) = mask;
                let unset_float_mask = (2u64.pow(36) - 1) ^ floating_bits;
                for float_mask in floating_combinations(floating_bits) {
                    mem.insert(((addr | set_bits) & unset_float_mask) | float_mask, value);
                }
            }
        }
    }
    mem.values().sum()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test2.txt")), 208);
}

#[test]
fn test_floating_combos() {
    assert_eq!(floating_combinations(0b11), vec![0b00, 0b01, 0b10, 0b11]);
    assert_eq!(floating_combinations(0b101), vec![0b0, 0b1, 0b100, 0b101]);
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 165);
}

#[test]
fn test_parse_mask() {
    assert_eq!(
        parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
        (
            0b1000000,
            0b111111111111111111111111111111111101,
            0b111111111111111111111111111110111101
        )
    );
}
