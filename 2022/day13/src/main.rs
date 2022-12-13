// Problem: https://adventofcode.com/2022/day/13

use std::cmp::Ordering;
use Ordering::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Number(u8),
    List(Vec<Item>),
}

fn parse(line: &str) -> Item {
    fn convert(val: &gjson::Value) -> Item {
        match val.kind() {
            gjson::Kind::Number => Item::Number(val.u8()),
            gjson::Kind::Array => Item::List(val.array().iter().map(convert).collect()),
            _ => panic!("invalid value"),
        }
    }
    convert(&gjson::parse(line))
}

fn items(item: &Item) -> Vec<Item> {
    if let Item::List(i) = item {
        i.clone()
    } else {
        panic!("not a list")
    }
}

fn as_list(item: &Item) -> Item {
    match item {
        Item::Number(n) => Item::List(vec![Item::Number(*n)]),
        Item::List(_) => item.clone(),
    }
}

fn num(item: &Item) -> u8 {
    match item {
        Item::Number(n) => *n,
        Item::List(_) => panic!("not a number"),
    }
}

fn compare(l: &Item, r: &Item) -> Ordering {
    if matches!(l, &Item::List(_)) && matches!(r, &Item::List(_)) {
        let l = items(l);
        let r = items(r);
        let len = std::cmp::min(l.len(), r.len());
        for i in 0..len {
            match compare(&l[i], &r[i]) {
                Less => return Less,
                Greater => return Greater,
                Equal => {}
            }
        }
        return l.len().cmp(&r.len());
    }

    if matches!(l, &Item::List(_)) || matches!(r, &Item::List(_)) {
        return compare(&as_list(l), &as_list(r));
    }

    num(l).cmp(&num(r))
}

fn part1(input: &str) -> usize {
    let pairs: Vec<(Item, Item)> = input
        .trim()
        .split("\n\n")
        .map(|ll| {
            let (l, r) = ll.split_once('\n').unwrap();
            (parse(l), parse(r))
        })
        .collect();

    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| {
            if compare(l, r) == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut pkgs: Vec<Item> = input
        .trim()
        .split("\n\n")
        .flat_map(|ll| ll.split('\n').map(parse))
        .collect();

    let a = parse("[[2]]");
    let b = parse("[[6]]");
    pkgs.push(a.clone());
    pkgs.push(b.clone());

    pkgs.sort_by(compare);

    let a_pos = pkgs.clone().iter().position(|x| x == &a).unwrap();
    let b_pos = pkgs.clone().iter().position(|x| x == &b).unwrap();

    (a_pos + 1) * (b_pos + 1)
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 140);
}

#[test]
fn test_compare() {
    assert_eq!(compare(&Item::Number(1), &Item::Number(1)), Equal);
    assert_eq!(compare(&Item::Number(1), &Item::Number(2)), Less);
    assert_eq!(compare(&Item::Number(2), &Item::Number(1)), Greater);
    assert_eq!(
        compare(
            &Item::List(vec![Item::Number(1)]),
            &Item::List(vec![Item::Number(1)]),
        ),
        Equal
    );
}
