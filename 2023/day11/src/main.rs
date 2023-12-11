// Problem: https://adventofcode.com/2023/day/11

use itertools::Itertools;

type Result = i64;

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn offset_at(v: usize, offset_list: &[usize], factor: usize) -> usize {
    offset_list.iter().filter(|o| **o < v).count() * (factor - 1)
}

fn find_expansion(input: &Input) -> (Vec<usize>, Vec<usize>) {
    let mut rows_to_expand = vec![];
    let mut cols_to_expand = vec![];
    (0..input.len()).for_each(|row| {
        if input[row].iter().all(|v| *v == '.') {
            rows_to_expand.push(row);
        }
    });
    for col in 0..input[0].len() {
        if input.iter().all(|v| v[col] == '.') {
            cols_to_expand.push(col);
        }
    }
    (rows_to_expand, cols_to_expand)
}

fn grid_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn part1(input: &Input) -> Result {
    part2(input, 2)
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 374);
}

fn part2(input: &Input, factor: usize) -> Result {
    let (rows_to_expand, cols_to_expand) = find_expansion(input);
    let mut galaxies = vec![];
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            if input[r][c] == '#' {
                galaxies.push((
                    (r + offset_at(r, &rows_to_expand, factor)) as i64,
                    (c + offset_at(c, &cols_to_expand, factor)) as i64,
                ));
            }
        }
    }
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| grid_distance(*a, *b))
        .sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input, 10), 1030);
    assert_eq!(part2(&input, 100), 8410);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input, 1_000_000));
}
