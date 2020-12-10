use std::collections::BTreeMap;

fn part1(inputs: &Vec<u32>) -> usize {
    let mut jolts = inputs.clone();
    jolts.push(0);
    jolts.sort();

    let diffs = jolts[..]
        .windows(2)
        .map(|c| c[1] as i32 - c[0] as i32)
        .collect::<Vec<i32>>();

    let one_jolts = diffs.iter().filter(|x| **x == 1).count();
    let three_jolts = diffs.iter().filter(|x| **x == 3).count() + 1;
    println!(
        "Part 1: {} * {} = {}",
        one_jolts,
        three_jolts,
        one_jolts * three_jolts
    );
    one_jolts * three_jolts
}

fn part2(inputs: &Vec<u32>) -> i64 {
    let mut jolts = inputs.clone();
    jolts.sort();
    let mut combinations: BTreeMap<i64, i64> = BTreeMap::new();
    let mut last: i64 = 0;
    combinations.insert(0, 1);
    for num in jolts.iter() {
        let n = *num as i64;
        let c1 = combinations.get(&(n - 1)).unwrap_or(&0);
        let c2 = combinations.get(&(n - 2)).unwrap_or(&0);
        let c3 = combinations.get(&(n - 3)).unwrap_or(&0);
        last = c1 + c2 + c3;
        combinations.insert(n, last);
    }
    println!("Part 2: {}", last);
    last
}

fn parse(s: &str) -> Vec<u32> {
    s.split('\n')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    let inputs = parse(include_str!("../in.txt"));
    part1(&inputs);
    part2(&inputs);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        assert_eq!(part1(&parse(include_str!("../test1.txt"))), 35);
        assert_eq!(part2(&parse(include_str!("../test1.txt"))), 8);
    }
    #[test]
    fn test_example2() {
        assert_eq!(part1(&parse(include_str!("../test2.txt"))), 220);
        assert_eq!(part2(&parse(include_str!("../test2.txt"))), 19208);
    }
}
