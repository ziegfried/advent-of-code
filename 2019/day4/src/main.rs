fn is_possible_password(n: usize) -> bool {
    let mut last = None;
    let mut double = false;
    for digit in n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
    {
        if Some(digit) == last {
            double = true;
        }
        if let Some(l) = last {
            if digit < l {
                return false;
            }
        }
        last = Some(digit);
    }
    return double;
}

fn part1(low: usize, high: usize) -> usize {
    let mut count: usize = 0;
    for n in low..=high {
        if is_possible_password(n) {
            count += 1
        }
    }
    count
}

fn is_possible_password2(n: usize) -> bool {
    let digits = n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut prev: u32 = digits[0];
    let mut group_size = 1;
    let mut found_double = false;
    for digit in digits.into_iter().skip(1) {
        if digit == prev {
            group_size += 1;
        } else {
            if digit < prev {
                return false;
            }
            if group_size == 2 {
                found_double = true;
            }
            prev = digit;
            group_size = 1;
        }
    }
    found_double || group_size == 2
}

fn part2(low: usize, high: usize) -> usize {
    let mut count: usize = 0;
    for n in low..=high {
        if is_possible_password2(n) {
            count += 1
        }
    }
    count
}

fn main() {
    println!("Part 1: {}", part1(138241, 674034));
    println!("Part 2: {}", part2(138241, 674034));
}

#[test]
fn test_is_possible_password() {
    assert_eq!(is_possible_password(111111), true);
    assert_eq!(is_possible_password(223450), false);
    assert_eq!(is_possible_password(123789), false);
}

#[test]
fn test_is_possible_password2() {
    assert_eq!(is_possible_password2(112233), true);
    assert_eq!(is_possible_password2(123444), false);
    assert_eq!(is_possible_password2(111122), true);
}
