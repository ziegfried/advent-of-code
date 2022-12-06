static PATTERN: [i32; 4] = [0, 1, 0, -1];

fn fft(value: &[i32]) -> Vec<i32> {
    value
        .iter()
        .enumerate()
        .map(|(round, _)| {
            (value
                .iter()
                .enumerate()
                .map(|(idx, v)| (v * PATTERN[((idx + 1) / (round + 1)) % 4]) % 10)
                .sum::<i32>()
                % 10)
                .abs()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let mut values = input
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();

    for _round in 0..100 {
        values = fft(&values[..]);
    }

    values[0..8]
        .iter()
        .map(|d| format!("{}", d))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn part2(_input: &str) -> usize {
    todo!()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1("80871224585914546619083218645595"), 24176176);
    assert_eq!(part1("19617804207202209144916044189917"), 73745418);
    assert_eq!(part1("69317163492948606335995924319873"), 52432133);
}

#[test]
fn test_part2() {
    assert_eq!(part2("03036732577212944063491565474664"), 84462026);
    assert_eq!(part2("02935109699940807407585447034323"), 78725270);
    assert_eq!(part2("03081770884921959731165446850517"), 53553731);
}
