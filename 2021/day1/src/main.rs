fn part1(input: &str) -> u32 {
    input
        .split('\n')
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .map(|values| if values[1] > values[0] { 1 } else { 0 })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .split('\n')
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|three_measures| three_measures.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|values| if values[1] > values[0] { 1 } else { 0 })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}
