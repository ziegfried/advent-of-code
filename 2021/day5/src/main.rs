use std::collections::HashMap;

type Point = (i32, i32);
type Line = (Point, Point);

fn parse_lines(input: &str) -> Vec<Line> {
    use nom::{
        bytes::complete::tag, character::complete::i32, character::complete::newline,
        multi::separated_list1, IResult,
    };
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
    fn lines(input: &str) -> IResult<&str, Vec<Line>> {
        let (input, lines) = separated_list1(newline, line)(input)?;
        Ok((input, lines))
    }
    let (remaining_input, lines) = lines(input).unwrap();
    assert_eq!(remaining_input, "");
    lines
}

fn draw_line(line: &Line, canvas: &mut HashMap<Point, usize>) {
    let ((x1, y1), (x2, y2)) = *line;
    let (dx, dy) = (x2 - x1, y2 - y1);
    let (x_step, y_step) = (dx.signum(), dy.signum());
    let mut x = x1;
    let mut y = y1;
    for _ in 0..=i32::max(dx.abs(), dy.abs()) {
        canvas.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
        x += x_step;
        y += y_step;
    }
}

fn part1(input: &str) -> usize {
    let lines = parse_lines(input);
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
    let lines = parse_lines(input);
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
