enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

use Command::*;

fn parse_command(line: &str) -> Command {
    let parts = line.split(' ').collect::<Vec<_>>();
    let amount = parts[1].parse::<i32>().unwrap();
    match parts[0] {
        "forward" => Forward(amount),
        "down" => Down(amount),
        "up" => Up(amount),
        _ => panic!("invalid command"),
    }
}

fn part1(input: &str) -> i32 {
    let (horizontal, depth) = input
        .split('\n')
        .map(parse_command)
        .fold((0, 0), |(h, d), cmd| match cmd {
            Forward(amount) => (h + amount, d),
            Down(amount) => (h, d + amount),
            Up(amount) => (h, d - amount),
        });

    horizontal * depth
}

fn part2(input: &str) -> i32 {
    let (horizontal, depth, _aim) =
        input
            .split('\n')
            .map(parse_command)
            .fold((0, 0, 0), |(h, d, a), cmd| match cmd {
                Forward(amount) => (h + amount, d + a * amount, a),
                Down(amount) => (h, d, a + amount),
                Up(amount) => (h, d, a - amount),
            });

    horizontal * depth
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 150);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 900);
}
