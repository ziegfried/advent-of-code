// Problem: https://adventofcode.com/2023/day/15

use sscanf::sscanf;

type Result = usize;

type Input = Vec<&'static str>;

fn parse_input(input: &'static str) -> Input {
    input.trim().split(',').collect()
}

// ------------------------------------------

fn hash(s: &str) -> usize {
    let mut hash = 0;
    for c in s.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn part1(input: &Input) -> Result {
    input.iter().map(|s| hash(s)).sum()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 1320);
}

// ------------------------------------------

#[derive(Debug)]
enum Op {
    Remove,
    Add,
}

fn part2(input: &Input) -> Result {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    for inst in input {
        let (label, op, focal_len) = if let Ok((l, v)) = sscanf!(inst, "{String}={usize}") {
            (l, Op::Add, v)
        } else {
            (sscanf!(inst, "{String}-").unwrap(), Op::Remove, 0)
        };
        let h = hash(label.as_str());
        match op {
            Op::Remove => {
                boxes[h] = boxes[h]
                    .iter()
                    .filter(|(l, _)| l != &label)
                    .cloned()
                    .collect::<Vec<_>>();
            }
            Op::Add => {
                if let Some(idx) = boxes[h].iter().position(|(l, _)| l == &label) {
                    boxes[h][idx] = (label.clone(), focal_len);
                } else {
                    boxes[h].push((label.clone(), focal_len));
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, (_, v))| (i + 1) * (j + 1) * v)
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 145);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
