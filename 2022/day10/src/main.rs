// Problem: https://adventofcode.com/2022/day/10

fn part1(input: &str) -> usize {
    struct Computer {
        cycle: usize,
        x: i32,
        result: usize,
    }
    let mut computer = Computer {
        cycle: 0,
        x: 1,
        result: 0,
    };
    fn inc(c: &mut Computer) {
        c.cycle += 1;
        if (c.cycle as i32 - 20) % 40 == 0 {
            c.result += c.x as usize * c.cycle;
        }
    }

    for inst in input.trim().lines() {
        if inst == "noop" {
            inc(&mut computer);
        } else {
            inc(&mut computer);
            let (_, amount) = inst.split_once(' ').unwrap();
            let amount: i32 = amount.parse().unwrap();
            inc(&mut computer);
            computer.x += amount;
        }
    }

    computer.result
}

fn part2(input: &str) -> String {
    struct Computer {
        cycle: usize,
        x: i32,
        crt: Vec<Vec<char>>,
    }
    let mut computer = Computer {
        cycle: 0,
        x: 1,
        crt: vec![vec![' '; 40]; 6],
    };
    fn inc(c: &mut Computer) {
        let row = c.cycle / 40;
        let pos = c.cycle % 40;
        let is_sprite_drawn = ((c.x - 1)..=(c.x + 1)).contains(&(pos as i32));
        c.crt[row][pos] = if is_sprite_drawn { '#' } else { '.' };
        c.cycle += 1;
    }

    for inst in input.trim().lines() {
        if inst == "noop" {
            inc(&mut computer);
        } else {
            inc(&mut computer);
            let (_, amount) = inst.split_once(' ').unwrap();
            let amount: i32 = amount.parse().unwrap();
            inc(&mut computer);
            computer.x += amount;
        }
    }

    computer
        .crt
        .iter()
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
        r"\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}
