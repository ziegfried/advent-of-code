// Problem: https://adventofcode.com/2022/day/25

fn from_snafu(n: impl AsRef<str>) -> i64 {
    n.as_ref()
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '=' => -2 * 5i64.pow(i as u32),
            '-' => -(5i64.pow(i as u32)),
            _ => c.to_digit(10).unwrap() as i64 * 5i64.pow(i as u32),
        })
        .sum()
}

fn to_snafu(n: i64) -> String {
    let digit_count: usize = (0..)
        .map(|digits| from_snafu("2".repeat(digits)))
        .take_while(|dv| *dv < n)
        .count();
    let mut digits = vec![];
    let mut n = n;
    for p in (1..digit_count).rev() {
        let p1 = 5i64.pow(p as u32);
        let min = from_snafu("=".repeat(p));
        let mut digit = -3;
        while (n - (digit + 1) * p1) >= min {
            digit += 1;
        }
        digits.push(digit);
        n -= digit * p1;
    }
    assert!((-2..=2).contains(&n));
    digits.push(n);
    digits
        .iter()
        .map(|d| match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("invalid snafu digit {}", d),
        })
        .collect()
}

fn part1(input: &str) -> String {
    let s: i64 = input.trim().lines().map(from_snafu).sum();
    to_snafu(s)
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), "2=-1=0");
}
