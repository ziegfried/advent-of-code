use nom::{bytes::complete::tag, character::complete::i32, IResult};
use std::collections::HashMap;

type Point = (i32, i32);
type Line = (Point, Point);

fn point(input: &str) -> IResult<&str, Point> {
    let (input, x) = i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = i32(input)?;
    Ok((input, (x, y)))
}

fn line(input: &str) -> IResult<&str, Line> {
    let (input, a) = point(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, b) = point(input)?;
    Ok((input, (a, b)))
}

fn parse_line(input: &str) -> Line {
    let (_, line) = line(input).unwrap();
    line
}

fn step(a: i32, b: i32) -> i32 {
    if a == b {
        0
    } else if a > b {
        -1
    } else {
        1
    }
}

fn draw_line(line: &Line, canvas: &mut HashMap<Point, usize>) {
    let ((x1, y1), (x2, y2)) = line;
    let x_step = step(*x1, *x2);
    let y_step = step(*y1, *y2);
    let count = i32::max((x2 - x1).abs(), (y2 - y1).abs()) + 1;
    let mut x = *x1;
    let mut y = *y1;
    for _ in 0..count {
        match canvas.get_mut(&(x, y)) {
            Some(value) => {
                *value += 1;
            }
            None => {
                canvas.insert((x, y), 1);
            }
        };
        x += x_step;
        y += y_step;
    }
}

fn part1(input: &str) -> usize {
    let lines = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut canvas = HashMap::<Point, usize>::new();
    for line in lines {
        let ((x1, y1), (x2, y2)) = line;
        if x1 == x2 || y1 == y2 {
            draw_line(&line, &mut canvas);
        }
    }
    canvas.values().filter(|&v| *v > 1).count()
}

fn part2(input: &str) -> usize {
    let lines = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut canvas = HashMap::<Point, usize>::new();
    for line in lines {
        draw_line(&line, &mut canvas);
    }
    canvas.values().filter(|&v| *v > 1).count()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 5);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 12);
}
