use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}
type FoldInstruction = (Axis, usize);
type Point = (usize, usize);
type Paper = HashSet<Point>;

fn parse_input(input: &str) -> (Paper, Vec<FoldInstruction>) {
    let (coords, instr) = input.split_once("\n\n").unwrap();

    let paper = coords
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect::<HashSet<Point>>();

    let instr = instr
        .lines()
        .map(|l| match l.split_once("=") {
            Some((pfx, val)) => match pfx {
                "fold along x" => (Axis::X, val.parse::<usize>().unwrap()),
                "fold along y" => (Axis::Y, val.parse::<usize>().unwrap()),
                _ => panic!(),
            },
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    (paper.clone(), instr)
}

fn fold_value(at: usize, val: usize) -> usize {
    if val > at {
        at - (val - at)
    } else {
        val
    }
}

fn fold((axis, fold_at): &FoldInstruction, paper: &Paper) -> Paper {
    paper
        .iter()
        .map(|&(x, y)| {
            (
                if *axis == Axis::X {
                    fold_value(*fold_at, x)
                } else {
                    x
                },
                if *axis == Axis::Y {
                    fold_value(*fold_at, y)
                } else {
                    y
                },
            )
        })
        .collect::<HashSet<_>>()
}

fn print_paper(p: &Paper) {
    let (min_x, max_x) = p.iter().map(|(x, _)| *x).minmax().into_option().unwrap();
    let (min_y, max_y) = p.iter().map(|(_, y)| *y).minmax().into_option().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if p.contains(&(x, y)) { "█" } else { " " });
        }
        println!();
    }
    println!();
}

fn part1(input: &str) -> usize {
    let (paper, fold_instructions) = parse_input(input);
    let final_paper = fold(fold_instructions.get(0).unwrap(), &paper);
    final_paper.len()
}

fn part2(input: &str) {
    let (paper, fold_instructions) = parse_input(input);
    let final_paper = fold_instructions
        .iter()
        .fold(paper, |paper, instr| fold(instr, &paper));
    print_paper(&final_paper);
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2:\n");
    part2(include_str!("in.txt"));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 17);
}
