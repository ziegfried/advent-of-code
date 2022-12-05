use itertools::Itertools;

fn part1(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = stacks
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for line in instructions.lines() {
        let (_, amount, _, src, _, dest) = line.split_whitespace().collect_tuple().unwrap();

        let amount: u32 = amount.parse().unwrap();
        let src: usize = src.parse().unwrap();
        let dest: usize = dest.parse().unwrap();

        let mut moving = vec![];
        for _ in 0..amount {
            let el = stacks[src - 1].pop().unwrap();
            moving.push(el);
        }
        for el in moving.into_iter() {
            stacks[dest - 1].push(el);
        }
    }

    let s: String = stacks.iter().map(|s| s[s.len()-1]).collect();
    
    s
}

fn part2(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = stacks
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for line in instructions.lines() {
        let (_, amount, _, src, _, dest) = line.split_whitespace().collect_tuple().unwrap();

        let amount: u32 = amount.parse().unwrap();
        let src: usize = src.parse().unwrap();
        let dest: usize = dest.parse().unwrap();

        let mut moving = vec![];
        for _ in 0..amount {
            let el = stacks[src - 1].pop().unwrap();
            moving.push(el);
        }
        for el in moving.into_iter().rev() {
            stacks[dest - 1].push(el);
        }
    }

    let s: String = stacks.iter().map(|s| s[s.len()-1]).collect();
    
    s
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), "CMZ");
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), "MCD");
}
