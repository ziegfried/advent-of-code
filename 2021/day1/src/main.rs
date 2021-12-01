use itertools::Itertools;

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows::<(u32, u32)>()
        .filter(|(a, b)| b > a)
        .count()
}

fn part2(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows::<(u32, u32, u32)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(u32, u32)>()
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 7);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 5);
}
