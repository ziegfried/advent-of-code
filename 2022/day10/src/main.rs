// Problem: https://adventofcode.com/2022/day/10

fn part1(input: &str) -> usize {
    struct System {
        cycle: usize,
        x: i32,
        result: usize,
    }
    let mut sys = System {
        cycle: 0,
        x: 1,
        result: 0,
    };
    fn inc(sys: &mut System) {
        sys.cycle += 1;
        if (sys.cycle as i32 - 20) % 40 == 0 {
            sys.result += sys.x as usize * sys.cycle;
        }
    }

    for inst in input.trim().lines() {
        if inst == "noop" {
            inc(&mut sys);
        } else {
            inc(&mut sys);
            let (_, amount) = inst.split_once(' ').unwrap();
            let amount: i32 = amount.parse().unwrap();
            inc(&mut sys);
            sys.x += amount;
        }
    }

    sys.result
}

fn part2(input: &str) -> String {
    struct System {
        cycle: usize,
        x: i32,
        crt: Vec<Vec<char>>,
    }
    let mut sys = System {
        cycle: 0,
        x: 1,
        crt: vec![vec![' '; 40]; 6],
    };
    fn inc(sys: &mut System) {
        let row = sys.cycle / 40;
        let pos = sys.cycle % 40;
        let is_sprite_drawn = ((sys.x - 1)..=(sys.x + 1)).contains(&(pos as i32));
        sys.crt[row][pos] = if is_sprite_drawn { '#' } else { '.' };
        sys.cycle += 1;
    }

    for inst in input.trim().lines() {
        if inst == "noop" {
            inc(&mut sys);
        } else {
            inc(&mut sys);
            let (_, amount) = inst.split_once(' ').unwrap();
            let amount: i32 = amount.parse().unwrap();
            inc(&mut sys);
            sys.x += amount;
        }
    }

    sys.crt
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
        r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}
