#![feature(linked_list_remove)]
// Problem: https://adventofcode.com/2022/day/20
use std::collections::LinkedList;

fn part1(input: &str) -> i32 {
    let input: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let len = input.len() as i32;
    let mut cur: LinkedList<(usize, i32)> = input.into_iter().enumerate().collect();
    for i in 0..len {
        let idx = cur.iter().position(|&(idx, _)| idx == i as usize).unwrap() as i32;
        let (mv_idx, mv_val) = cur.remove(idx as usize);
        let target = (idx + mv_val).rem_euclid(len - 1);
        let mut tail = cur.split_off(target as usize);
        cur.push_back((mv_idx, mv_val));
        cur.append(&mut tail);
    }
    let zero_idx = cur.iter().position(|(_, v)| *v == 0).unwrap();
    vec![1000, 2000, 3000]
        .iter()
        .map(|p| {
            let idx = (p + zero_idx) as i32 % len;
            let (_, val) = cur.iter().nth(idx as usize).unwrap();
            val
        })
        .sum::<i32>()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 3);
}

fn part2(input: &str) -> i64 {
    let key: i64 = 811589153;
    let input: Vec<_> = input
        .trim()
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * key)
        .collect();
    let len = input.len() as i64;
    let mut cur: LinkedList<(usize, i64)> = input.into_iter().enumerate().collect();
    for _ in 0..10 {
        for i in 0..len {
            let idx = cur.iter().position(|&(idx, _)| idx == i as usize).unwrap() as i64;
            let (mv_idx, mv_val) = cur.remove(idx as usize);
            let target = (idx + mv_val).rem_euclid(len - 1);
            let mut tail = cur.split_off(target as usize);
            cur.push_back((mv_idx, mv_val));
            cur.append(&mut tail);
        }
    }
    let zero_idx = cur.iter().position(|(_, v)| *v == 0).unwrap();
    vec![1000, 2000, 3000]
        .iter()
        .map(|p| {
            let idx = (p + zero_idx) as i64 % len;
            let (_, val) = cur.iter().nth(idx as usize).unwrap();
            val
        })
        .sum::<i64>()
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 1623178306);
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}
