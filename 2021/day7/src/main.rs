fn part1(input: &str) -> u64 {
    let positions = input
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|i| {
            positions
                .iter()
                .map(|&n| if n > i { n - i } else { i - n })
                .sum::<u64>()
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let positions = input
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|i| {
            positions
                .iter()
                .map(|&n| if n > i { n - i } else { i - n })
                .map(|dist| (dist * (dist + 1)) / 2)
                .sum::<u64>()
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
