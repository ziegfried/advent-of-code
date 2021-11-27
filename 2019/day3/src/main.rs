use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

fn manhattan_distance(point: (isize, isize)) -> isize {
    point.0.abs() + point.1.abs()
}

fn parse_intructions(s: &str) -> Vec<(isize, isize, isize)> {
    s.split(',')
        .map(|inst| {
            let n = inst[1..].parse::<isize>().unwrap();
            match &inst[0..1] {
                "U" => (n, 1, 0),
                "D" => (n, -1, 0),
                "L" => (n, 0, -1),
                "R" => (n, 0, 1),
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> isize {
    let paths = input.split('\n').map(parse_intructions).collect::<Vec<_>>();
    let mut map: HashSet<(isize, isize)> = HashSet::new();

    let mut x: isize = 0;
    let mut y: isize = 0;

    for inst in &paths[0] {
        let (n, dx, dy) = inst;
        for _ in 0..*n {
            x += dx;
            y += dy;
            map.insert((x, y));
        }
    }

    let mut result: isize = isize::MAX;
    let mut x: isize = 0;
    let mut y: isize = 0;
    for inst in &paths[1] {
        let (n, dx, dy) = inst;
        for _ in 0..*n {
            x += dx;
            y += dy;
            if map.contains(&(x, y)) {
                result = cmp::min(manhattan_distance((x, y)), result);
            }
        }
    }

    result
}

fn part2(input: &str) -> isize {
    let paths = input.split('\n').map(parse_intructions).collect::<Vec<_>>();
    let mut map: HashMap<(isize, isize), isize> = HashMap::new();

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut steps: isize = 0;

    for inst in &paths[0] {
        let (n, dx, dy) = inst;
        for _ in 0..*n {
            x += dx;
            y += dy;
            steps += 1;
            if !map.contains_key(&(x, y)) {
                map.insert((x, y), steps);
            }
        }
    }

    let mut result: isize = isize::MAX;
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut steps: isize = 0;
    for inst in &paths[1] {
        let (n, dx, dy) = inst;
        for _ in 0..*n {
            x += dx;
            y += dy;
            steps += 1;
            if let Some(other_steps) = map.get(&(x, y)) {
                result = cmp::min(steps + other_steps, result);
            }
        }
    }

    result
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test1.txt")), 159);
    assert_eq!(part1(include_str!("test2.txt")), 135);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test1.txt")), 610);
    assert_eq!(part2(include_str!("test2.txt")), 410);
}
