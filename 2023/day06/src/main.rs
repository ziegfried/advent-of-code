// Problem: https://adventofcode.com/2023/day/6

type Result = usize;

type Input = (Vec<usize>, Vec<usize>);

fn parse_input(input: &str) -> Input {
    let (line1, line2) = input.split_once('\n').unwrap();
    let time_str = line1.split_once(':').unwrap().1;
    let dist_str = line2.split_once(':').unwrap().1;
    (
        time_str
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
        dist_str
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
    )
}

fn race_dist(speed: usize, time: usize) -> usize {
    let rem = time - speed;
    rem * speed
}

fn count_winners(time: usize, dist: usize) -> usize {
    (1..=time)
        .filter(|speed| race_dist(*speed, time) > dist)
        .count()
}

fn part1((times, dist): &Input) -> Result {
    times
        .iter()
        .zip(dist.iter())
        .map(|(time, dist)| count_winners(*time, *dist))
        .product()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    dbg!(&input);
    assert_eq!(part1(&input), 288);
}

fn list_to_number(list: Vec<usize>) -> usize {
    let s = list
        .iter()
        .map(|n| format!("{}", n))
        .collect::<Vec<String>>()
        .join("");
    s.parse::<usize>().unwrap()
}

fn part2((times, dist): &Input) -> Result {
    count_winners(list_to_number(times.clone()), list_to_number(dist.clone()))
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 71503);
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
