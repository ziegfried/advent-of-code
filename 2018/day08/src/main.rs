// Problem: https://adventofcode.com/2018/day/8

type Result = i32;

type Input = Vec<i32>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

// ------------------------------------------

fn sum_meta(it: &mut dyn Iterator<Item = &i32>) -> i32 {
    let child_count = it.next().unwrap();
    let meta_count = it.next().unwrap();
    let child_sum: i32 = (0..*child_count).map(|_| sum_meta(it)).sum();
    let meta_sum: i32 = (0..*meta_count).map(|_| it.next().unwrap()).sum();
    child_sum + meta_sum
}

fn part1(input: &Input) -> Result {
    let mut it = input.iter();
    sum_meta(&mut it)
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 138);
}

// ------------------------------------------

fn node_value(it: &mut dyn Iterator<Item = &i32>) -> i32 {
    let child_count = *it.next().unwrap();
    let meta_count = *it.next().unwrap();

    if child_count == 0 {
        return (0..meta_count).map(|_| *it.next().unwrap()).sum();
    }

    let children: Vec<i32> = (0..child_count).map(|_| node_value(it)).collect();
    let meta: Vec<usize> = (0..meta_count)
        .map(|_| *it.next().unwrap())
        .filter(|v| *v > 0)
        .map(|v| v as usize)
        .collect();

    meta.iter().filter_map(|idx| children.get(*idx - 1)).sum()
}

fn part2(input: &Input) -> Result {
    let mut it = input.iter();
    node_value(&mut it)
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 66);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
