// Problem: https://adventofcode.com/2022/day/10

use std::cell::Cell;

fn part1(input: &str) -> usize {
    let mut cycle = 0;
    let x: Cell<i32> = Cell::new(1);
    let mut result = 0;

    let mut inc = || {
        cycle += 1;
        if (cycle as i32 - 20) % 40 == 0 {
            result += x.get() as usize * cycle;
        }
    };

    for inst in input.trim().lines() {
        if inst == "noop" {
            inc();
        } else {
            inc();
            let (_, amount) = inst.split_once(' ').unwrap();
            let amount: i32 = amount.parse().unwrap();
            inc();
            x.replace(x.get() + amount);
        }
    }

    result
}

fn part2(input: &str) -> String {
    let mut cycle = 0;
    let x: Cell<i32> = Cell::new(1);
    let mut crt = vec![vec![' '; 40]; 6];

    let mut inc = || {
        let x = x.get();
        let row = cycle / 40;
        let pos = cycle % 40;
        let is_sprite_drawn = ((x - 1)..=(x + 1)).contains(&(pos as i32));
        crt[row][pos] = if is_sprite_drawn { '#' } else { '.' };
        cycle += 1;
    };

    for inst in input.trim().lines() {
        if inst == "noop" {
            inc();
        } else {
            inc();
            let (_, amount) = inst.split_once(' ').unwrap();
            let amount: i32 = amount.parse().unwrap();
            inc();
            x.replace(x.get() + amount);
        }
    }

    crt.iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2:\n{}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 13140);
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(include_str!("test.txt")),
        r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}
