// Problem: https://adventofcode.com/2023/day/2

use sscanf::sscanf;

type Result = usize;

#[derive(Debug)]
struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}
type Input = Vec<(usize, Vec<GameSet>)>;

fn parse_game_set(input: &str) -> GameSet {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for part in input.split(", ") {
        let (amount, color) = sscanf!(part, "{usize} {String}").unwrap();
        if color == "red" {
            red = amount;
        } else if color == "green" {
            green = amount;
        } else if color == "blue" {
            blue = amount;
        } else {
            panic!("unexpected color {}", color);
        }
    }
    GameSet { red, green, blue }
}

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (id, games_str) = sscanf!(line, "Game {usize}: {String}").unwrap();
            (id, games_str.split("; ").map(parse_game_set).collect())
        })
        .collect()
}

// ------------------------------------------

fn part1(input: &Input) -> Result {
    input
        .iter()
        .map(|(id, sets)| {
            if sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
            {
                *id
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 8);
}

// ------------------------------------------

fn part2(input: &Input) -> Result {
    input
        .iter()
        .map(|(_, sets)| {
            let red = sets.iter().map(|s| s.red).max().unwrap_or(1);
            let green = sets.iter().map(|s| s.green).max().unwrap_or(1);
            let blue = sets.iter().map(|s| s.blue).max().unwrap_or(1);
            red * green * blue
        })
        .sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 2286);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
