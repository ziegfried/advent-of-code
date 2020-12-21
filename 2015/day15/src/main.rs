#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::cmp::max;

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse_line(line: &str) -> Ingredient {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+?): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    }
    let m = RE.captures(line).unwrap();
    Ingredient {
        name: String::from(&m[1]),
        capacity: *&m[2].parse::<isize>().unwrap(),
        durability: *&m[3].parse::<isize>().unwrap(),
        flavor: *&m[4].parse::<isize>().unwrap(),
        texture: *&m[5].parse::<isize>().unwrap(),
        calories: *&m[6].parse::<isize>().unwrap(),
    }
}

fn possible_amounts(n: usize, amount: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = vec![];
    if n == 2 {
        for a in 0..=amount {
            result.push(vec![a, amount - a]);
        }
    } else {
        for b in 0..=amount {
            for sub in possible_amounts(n - 1, amount - b) {
                let mut v = vec![b];
                let mut rest = sub;
                v.append(&mut rest);
                result.push(v);
            }
        }
    }
    result
}

fn score_of(amounts: &Vec<usize>, ingredients: &Vec<Ingredient>) -> (isize, isize) {
    let score = amounts
        .iter()
        .map(|amount| *amount as isize)
        .zip(ingredients.clone().iter())
        .map(|(amount, ingredient)| {
            vec![
                (amount * ingredient.capacity),
                (amount * ingredient.durability),
                (amount * ingredient.flavor),
                (amount * ingredient.texture),
            ]
        })
        .fold(vec![0, 0, 0, 0], |a, b| {
            (0..4).map(|i| a[i] + b[i]).collect::<Vec<_>>()
        })
        .iter()
        .fold(1, |a, b| max(0, a * b));
    let calories = amounts
        .iter()
        .map(|amount| *amount as isize)
        .zip(ingredients.clone().iter())
        .map(|(amount, ingredient)| amount * ingredient.calories)
        .sum();
    (score, calories)
}

fn part1(s: &str) -> isize {
    let input = s.lines().map(parse_line).collect::<Vec<Ingredient>>();
    possible_amounts(input.len(), 100)
        .iter()
        .map(|d| score_of(d, &input).0)
        .max()
        .unwrap()
}

fn part2(s: &str) -> isize {
    let input = s.lines().map(parse_line).collect::<Vec<Ingredient>>();
    possible_amounts(input.len(), 100)
        .iter()
        .map(|d| score_of(d, &input))
        .filter(|(_, calories)| *calories == 500)
        .map(|(score, _)| score)
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 62842880);
    assert_eq!(part2(include_str!("test.txt")), 57600000);
}

#[test]
fn test_score_of() {
    let ingredients = include_str!("test.txt")
        .lines()
        .map(parse_line)
        .collect::<Vec<Ingredient>>();
    assert_eq!(score_of(&vec![44, 56], &ingredients).0, 62842880);
    assert!(score_of(&vec![1, 99], &ingredients).0 <= 62842880);
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"),
        Ingredient {
            name: String::from("Butterscotch"),
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8
        }
    );
}
