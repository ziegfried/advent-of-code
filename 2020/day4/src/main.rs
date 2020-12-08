#[macro_use]
extern crate lazy_static;
extern crate regex;
use std::collections::HashSet;

fn valid_haircolor(val: &str) -> bool {
    lazy_static! {
        static ref VALID_HCL: regex::Regex = regex::Regex::new("^#[0-9a-f]{6}$").unwrap();
    }
    VALID_HCL.is_match(val)
}

fn valid_eye_color(val: &str) -> bool {
    lazy_static! {
        static ref VALID_ECL: regex::Regex =
            regex::Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }
    VALID_ECL.is_match(val)
}

fn valid_passport_id(val: &str) -> bool {
    lazy_static! {
        static ref VALID_PID: regex::Regex = regex::Regex::new(r"^\d{9}$").unwrap();
    }
    VALID_PID.is_match(val)
}

fn number_in_range(val: &str, min: usize, max: usize) -> bool {
    if let Ok(n) = val.parse::<usize>() {
        n >= min && n <= max
    } else {
        false
    }
}

fn valid_height(val: &str) -> bool {
    lazy_static! {
        static ref HEIGHT_RE: regex::Regex = regex::Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }

    match HEIGHT_RE.captures(val) {
        Some(m) => match &m[2] {
            "cm" => number_in_range(&m[1], 150, 193),
            "in" => number_in_range(&m[1], 59, 76),
            _ => false,
        },
        None => false,
    }
}

fn main() {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let sep = regex::Regex::new(r"\s+").unwrap();

    let mut valid_count1 = 0;
    let mut valid_count2 = 0;

    for chunk in include_str!("../in.txt").split("\n\n") {
        let parts: Vec<&str> = sep.split(chunk.trim()).into_iter().collect();
        let mut field_names = HashSet::new();
        let mut valid_record = true;

        for part in parts {
            let split: Vec<&str> = part.splitn(2, ':').into_iter().collect();
            let field_name = split[0];
            field_names.insert(field_name);
            let valid_value = match field_name {
                "byr" => number_in_range(split[1], 1920, 2002),
                "iyr" => number_in_range(split[1], 2010, 2020),
                "eyr" => number_in_range(split[1], 2020, 2030),
                "hgt" => valid_height(split[1]),
                "hcl" => valid_haircolor(split[1]),
                "ecl" => valid_eye_color(split[1]),
                "pid" => valid_passport_id(split[1]),
                "cid" => true,
                _ => true,
            };
            if !valid_value {
                valid_record = false;
            }
        }
        let has_req_fields = required.iter().all(|f| field_names.contains(f));

        if has_req_fields {
            valid_count1 += 1;
        }

        if valid_record {
            if has_req_fields {
                valid_count2 += 1;
            }
        }
    }

    println!("Part 1: {}", valid_count1);
    println!("Part 2: {}", valid_count2);
}
