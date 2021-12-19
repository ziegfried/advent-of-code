use itertools::Itertools;

fn part1(input: &str) -> i64 {
    let positions = input
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();
    (min..=max)
        .map(|i| positions.iter().map(|&n| i64::abs(n - i)).sum::<i64>())
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let positions = input
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();
    (min..=max)
        .map(|i| {
            positions
                .iter()
                .map(|&n| i64::abs(n - i))
                .map(|dist| (dist * (dist + 1)) / 2)
                .sum::<i64>()
        })
        .min()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 37);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 168);
}
