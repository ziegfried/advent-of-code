fn parse_pair(s: &str) -> (u32, u32) {
    let (l, r) = s.split_once('-').unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(',').unwrap();
            (parse_pair(p1), parse_pair(p2))
        })
        .filter(|((x1, y1), (x2, y2))| (x1 <= x2 && y1 >= y2) || (x2 <= x1 && y2 >= y1))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(',').unwrap();
            (parse_pair(p1), parse_pair(p2))
        })
        .filter(|((x1, y1), (x2, y2))| std::cmp::max(x1, x2) <= std::cmp::min(y1, y2))
        .count()
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("in.txt")));
    println!("Part 2: {:?}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 2);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 4);
}
