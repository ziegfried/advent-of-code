#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn shape_score(shape: Shape) -> usize {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn outcome_score(outcome: Outcome) -> usize {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    }
}

fn outcome(me: Shape, opp: Shape) -> Outcome {
    use Outcome::*;
    if me == opp {
        Draw
    } else {
        match (me, opp) {
            (Shape::Rock, Shape::Scissors) => Win,
            (Shape::Paper, Shape::Rock) => Win,
            (Shape::Scissors, Shape::Paper) => Win,
            _ => Lose,
        }
    }
}

fn parse_shape(s: &str) -> Shape {
    match s {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("Invalid shape"),
    }
}

fn parse_outcome(s: &str) -> Outcome {
    match s {
        "Z" => Outcome::Win,
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        _ => panic!("Invalid outcome"),
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (opp, me) = line.split_once(' ').unwrap();
            let (me, opp) = (parse_shape(me), parse_shape(opp));
            shape_score(me) + outcome_score(outcome(me, opp))
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (opp, expected_outcome) = line.split_once(' ').unwrap();
            let opp = parse_shape(opp);
            let expected_outcome = parse_outcome(expected_outcome);

            let all_shapes = vec![Shape::Rock, Shape::Paper, Shape::Scissors];
            let me = *all_shapes
                .iter()
                .find(|&me| outcome(*me, opp) == expected_outcome)
                .unwrap();

            shape_score(me) + outcome_score(outcome(me, opp))
        })
        .sum()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("in.txt")));
    println!("Part 2: {:?}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 15);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 12);
}
