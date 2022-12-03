fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|block| {
            block
                .trim()
                .lines()
                .map(|x| x.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut elves: Vec<usize> = input
        .split("\n\n")
        .map(|block| {
            block
                .trim()
                .lines()
                .map(|x| x.parse::<usize>().unwrap())
                .sum()
        })
        .collect();
    elves.sort();
    elves[elves.len() - 3..].iter().sum()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("in.txt")));
    println!("Part 2: {:?}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 24000);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 45000);
}
