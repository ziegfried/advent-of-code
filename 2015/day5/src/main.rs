fn is_nice(s: &str) -> bool {
    let vowels = s
        .chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count();
    let has_double_letter = s
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|c| c[0] == c[1]);
    let has_naughty_seq = s.chars().collect::<Vec<char>>().windows(2).any(|c| {
        match c.into_iter().collect::<String>().as_str() {
            "ab" | "cd" | "pq" | "xy" => true,
            _ => false,
        }
    });
    vowels >= 3 && has_double_letter && !has_naughty_seq
}

fn is_nice2(s: &str) -> bool {
    let x = String::from(s);
    let has_double_pair =
        s.chars()
            .collect::<Vec<char>>()
            .windows(2)
            .enumerate()
            .any(|(idx, chars)| {
                let pair = chars.into_iter().collect::<String>();
                if let Some(f) = x.rfind(&pair) {
                    f > idx + 1
                } else {
                    false
                }
            });
    let has_pair_with_blank = s
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|c| c[0] == c[2] && c[0] != c[1]);
    has_double_pair && has_pair_with_blank
}

fn main() {
    let nice_count = include_str!("../in.txt")
        .split("\n")
        .map(|line| is_nice(line))
        .fold(0, |count, naughty| count + if naughty { 1 } else { 0 });

    println!("Part 1: {}", nice_count);

    let nice2_count = include_str!("../in.txt")
        .split("\n")
        .map(|line| is_nice2(line))
        .fold(0, |count, naughty| count + if naughty { 1 } else { 0 });
    println!("Part 2: {}", nice2_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_naughty() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_is_naughty2() {
        assert_eq!(is_nice2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice2("xxyxx"), true);
        assert_eq!(is_nice2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice2("ieodomkazucvgmuy"), false);
    }
}
