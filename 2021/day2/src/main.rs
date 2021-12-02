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
    let commands = input.split('\n').map(parse_command);
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Forward(amount) => {
                horizontal += amount;
            }
            Down(amount) => {
                depth += amount;
            }
            Up(amount) => {
                depth -= amount;
            }
        }
    }

    horizontal * depth
}

fn part2(input: &str) -> i32 {
    let commands = input.split('\n').map(parse_command);
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Forward(amount) => {
                horizontal += amount;
                depth += aim * amount;
            }
            Down(amount) => {
                aim += amount;
            }
            Up(amount) => {
                aim -= amount;
            }
        }
    }

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
