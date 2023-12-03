// Problem: https://adventofcode.com/2023/day/3

use std::collections::HashMap;

type Result = usize;

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

// ------------------------------------------

fn has_adjacent_symbol(row: usize, start: usize, end: usize, input: &Input) -> bool {
    let row = row as i32;
    let start = start as i32;
    let end = end as i32;
    for r in (row - 1)..=(row + 1) {
        for c in (start - 1)..(end + 1) {
            if r >= 0 && c >= 0 {
                if let Some(row) = input.get(r as usize) {
                    if let Some(&ch) = row.get(c as usize) {
                        if ch != '.' && !ch.is_ascii_digit() {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn part1(input: &Input) -> Result {
    let row_count = input.len();
    let col_count = input[0].len();
    let mut sum = 0;

    for i in 0..row_count {
        let mut j = 0;
        while j < col_count {
            if input[i][j].is_ascii_digit() {
                let start = j;
                while j < col_count && input[i][j].is_ascii_digit() {
                    j += 1;
                }
                let end = j;

                if has_adjacent_symbol(i, start, end, input) {
                    sum += input[i][start..end]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                }
            }
            j += 1;
        }
    }

    sum
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 4361);
}

// ------------------------------------------

fn get_adjacent_stars(row: usize, start: usize, end: usize, input: &Input) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let row = row as i32;
    let start = start as i32;
    let end = end as i32;
    for r in (row - 1)..=(row + 1) {
        for c in (start - 1)..(end + 1) {
            if r >= 0 && c >= 0 {
                if let Some(row) = input.get(r as usize) {
                    if let Some(&ch) = row.get(c as usize) {
                        if ch == '*' {
                            result.push((r as usize, c as usize));
                        }
                    }
                }
            }
        }
    }
    result
}

fn part2(input: &Input) -> Result {
    let row_count = input.len();
    let col_count = input[0].len();
    let mut star_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for i in 0..row_count {
        let mut j = 0;
        while j < col_count {
            if input[i][j].is_ascii_digit() {
                let start = j;
                while j < col_count && input[i][j].is_ascii_digit() {
                    j += 1;
                }
                let end = j;

                let n = input[i][start..end]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();

                for loc in get_adjacent_stars(i, start, end, input) {
                    star_map.entry(loc).or_default().push(n);
                }
            }
            j += 1;
        }
    }

    star_map
        .values()
        .filter_map(|values| {
            if values.len() > 1 {
                Some(values[0] * values[1])
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 467835);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
