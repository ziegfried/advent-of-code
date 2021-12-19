extern crate regex;
use regex::Regex;

fn count(ch: &str, s: &str) -> usize {
    let mut count = 0;
    let cmp = ch.chars().next().unwrap();
    for cur in s.chars() {
        if cur == cmp {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    let mut valid_count1 = 0;
    let mut valid_count2 = 0;

    for text in include_str!("../in.txt").split('\n') {
        let m = re.captures(text).unwrap();
        let min = String::from(&m[1]).parse::<usize>().expect("PARSEINT ERR");
        let max = String::from(&m[2]).parse::<usize>().expect("PARSEINT ERR");
        let cnt = count(&m[3], &m[4]);
        if cnt >= min && cnt <= max {
            valid_count1 += 1;
        }

        let ch1 = &m[4].chars().nth(min - 1);
        let ch2 = &m[4].chars().nth(max - 1);
        let cmp = &m[3].chars().nth(0);

        if (ch1 == cmp && ch2 != cmp) || (ch1 != cmp && ch2 == cmp) {
            valid_count2 += 1;
        }
    }

    println!("Part 1: {}", valid_count1);
    println!("Part 2: {}", valid_count2);
}
