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
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::character::complete::u32;
    use nom::multi::separated_list1;
    use nom::IResult;

    fn point(input: &str) -> IResult<&str, Point> {
        let (input, x) = u32(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = u32(input)?;
        Ok((input, (x as usize, y as usize)))
    }

    fn paper(input: &str) -> IResult<&str, Paper> {
        let (input, points) = separated_list1(newline, point)(input)?;
        Ok((input, points.iter().map(|&v| v).collect::<Paper>()))
    }

    fn instruction(input: &str) -> IResult<&str, FoldInstruction> {
        let (input, _) = tag("fold along ")(input)?;
        let (input, axis_str) = alt((tag("x"), tag("y")))(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, value) = u32(input)?;
        Ok((
            input,
            (
                match axis_str {
                    "x" => Axis::X,
                    "y" => Axis::Y,
                    _ => panic!("unexpected axis {}", axis_str),
                },
                value as usize,
            ),
        ))
    }

    fn instructions(input: &str) -> IResult<&str, Vec<FoldInstruction>> {
        let (input, result) = separated_list1(newline, instruction)(input)?;
        Ok((input, result))
    }

    fn full_input(input: &str) -> IResult<&str, (Paper, Vec<FoldInstruction>)> {
        let (input, paper) = paper(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = newline(input)?;
        let (input, instr) = instructions(input)?;
        Ok((input, (paper, instr)))
    }

    let (input, result) = full_input(input).expect("invalid input");
    assert_eq!(input, "");
    result
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
