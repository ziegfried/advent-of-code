// Problem: https://adventofcode.com/2023/day/1

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|d| d.is_ascii_digit()).collect();
            format!("{}{}", digits[0], digits[digits.len() - 1])
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 142);
    assert_eq!(part1(include_str!("input.txt")), 56465);
}

fn starting_word_digit(s: &str) -> Option<usize> {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    if let Some((o, _)) = words.iter().enumerate().find(|(_, w)| s.starts_with(*w)) {
        return Some(o + 1);
    }
    None
}

fn find_digits(line: &str) -> Vec<usize> {
    line.chars()
        .enumerate()
        .filter_map(|(idx, d)| {
            if d.is_ascii_digit() {
                return Some(d as usize - '0' as usize);
            }
            starting_word_digit(&line[idx..])
        })
        .collect()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(find_digits)
        .map(|digits| digits[0] * 10 + digits[digits.len() - 1])
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test2.txt")), 281);
    assert_eq!(part2(include_str!("input.txt")), 55902);
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}
