use std::char;

fn from_str(password: &str) -> u64 {
    let mut cur: u64 = 0;
    for c in password.chars() {
        cur = cur * 26 + (c as u32 - 'a' as u32) as u64;
    }
    cur
}

fn to_str(password: u64) -> String {
    let mut chars = vec![];
    let mut cur = password;
    while cur > 0 {
        let c = cur % 26;
        cur = cur / 26;
        let s = char::from_u32('a' as u32 + c as u32).unwrap();
        chars.push(s);
    }
    return chars.into_iter().rev().collect::<String>();
}

fn has_straight(pwd: &str) -> bool {
    pwd.chars().collect::<Vec<_>>().windows(3).any(|w| {
        let a = w[0] as u32;
        let b = w[1] as u32;
        let c = w[2] as u32;
        c - 1 == b && b - 1 == a
    })
}

fn has_pairs(pwd: &str) -> bool {
    let chars = pwd.chars().collect::<Vec<_>>();
    let pairs = chars
        .windows(2)
        .enumerate()
        .filter(|(_, w)| w[0] == w[1])
        .collect::<Vec<_>>();
    pairs.windows(2).filter(|w| w[1].0 - w[0].0 > 1).count() >= 1
}

fn has_no_invalid_chars(pwd: &str) -> bool {
    pwd.chars().all(|c| match c {
        'i' | 'o' | 'l' => false,
        _ => true,
    })
}

fn is_valid(pwd: &str) -> bool {
    has_no_invalid_chars(pwd) && has_straight(pwd) && has_pairs(pwd)
}

fn main() {
    let mut cur = from_str("vzbxkghb") + 1;
    while !is_valid(to_str(cur).as_str()) {
        cur += 1;
    }
    println!("{}", to_str(cur));
    cur += 1;
    while !is_valid(to_str(cur).as_str()) {
        cur += 1;
    }
    println!("{}", to_str(cur));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_has_straight() {
        assert_eq!(has_straight("abc"), true);
        assert_eq!(has_straight("aab"), false);
    }

    #[test]
    fn test_has_pairs() {
        assert_eq!(has_pairs("aabb"), true);
        assert_eq!(has_pairs("aaab"), false);
        assert_eq!(has_pairs("aaabb"), true);
    }

    #[test]
    fn test_sample_1() {
        let password = "hijklmmn";
        assert_eq!(has_straight(password), true);
        assert_eq!(has_no_invalid_chars(password), false);
    }

    #[test]
    fn test_from_and_to_str() {
        let cur = from_str("hijklmmn");
        dbg!(cur);
        assert_eq!(to_str(cur + 1), "hijklmmo");
    }
}
