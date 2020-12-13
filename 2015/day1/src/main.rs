#[derive(Debug, PartialEq)]
enum Parenthesis {
    Open,
    Close,
}
use Parenthesis::*;

fn main() {
    let instructions = include_str!("../in.txt").trim().chars().map(|c| match c {
        '(' => Open,
        ')' => Close,
        c => panic!(format!("nah: {:?}", c)),
    });

    let part1 = instructions.clone().fold(0, |floor, p| match p {
        Open => floor + 1,
        Close => floor - 1,
    });
    println!("Part 1: {}", part1);

    let mut floor = 0;
    for (i, p) in instructions.enumerate() {
        floor = match p {
            Open => floor + 1,
            Close => floor - 1,
        };
        if floor < 0 {
            println!("Part 2: {}", i + 1);
            return;
        }
    }
}
