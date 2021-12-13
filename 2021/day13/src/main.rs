use std::collections::HashSet;

#[derive(Debug)]
enum FoldInstruction {
    AlongX(usize),
    AlongY(usize),
}

type Point = (usize, usize);

fn parse_input(input: &str) -> (Vec<Point>, Vec<FoldInstruction>) {
    let (coords, instr) = input.split_once("\n\n").unwrap();

    let paper = coords
        .lines()
        .map(|line| {
            let xy = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (xy[0], xy[1])
        })
        .collect::<Vec<Point>>();

    let instr = instr
        .lines()
        .map(|l| match l.split_once("=") {
            Some((pfx, val)) => match pfx {
                "fold along x" => FoldInstruction::AlongX(val.parse::<usize>().unwrap()),
                "fold along y" => FoldInstruction::AlongY(val.parse::<usize>().unwrap()),
                _ => panic!(),
            },
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    (paper.clone(), instr)
}

fn fold_at(at: usize, val: usize) -> usize {
    if val > at {
        at - (val - at)
    } else {
        val
    }
}

fn fold(instruction: &FoldInstruction, paper: &HashSet<Point>) -> HashSet<Point> {
    paper
        .iter()
        .map(|&(x, y)| {
            let nx = match instruction {
                &FoldInstruction::AlongX(at) => fold_at(at, x),
                _ => x,
            };
            let ny = match instruction {
                &FoldInstruction::AlongY(at) => fold_at(at, y),
                _ => y,
            };
            (nx, ny)
        })
        .collect::<HashSet<_>>()
}

fn print_paper(p: &HashSet<Point>) {
    let min_x = p.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = p.iter().map(|(_, y)| *y).min().unwrap();
    let max_x = p.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = p.iter().map(|(_, y)| *y).max().unwrap();
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
    let paper = paper.iter().map(|p| *p).collect::<HashSet<Point>>();
    let final_paper = fold(fold_instructions.get(0).unwrap(), &paper);
    final_paper.len()
}

fn part2(input: &str) {
    let (paper, fold_instructions) = parse_input(input);
    let paper = paper.iter().map(|p| *p).collect::<HashSet<Point>>();
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
