#[macro_use]
extern crate lazy_static;
extern crate regex;

#[derive(Debug, PartialEq)]
struct Range((usize, usize), (usize, usize));

impl Range {
    fn contains(&self, (x,y): (usize,usize)) -> bool {
        (x >= self.0.0 && x <= self.1.0) && (y >= self.0.1 && y <= self.1.1)
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Rule {
    action: Action,
    range: Range,
}

fn parse_action(s: &str) -> Rule {
    lazy_static! {
        static ref RE: regex::Regex =
            regex::Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")
                .unwrap();
    }
    let m = RE.captures(s).unwrap();

    let range = Range(
        (
            m[2].parse::<usize>().unwrap(),
            m[3].parse::<usize>().unwrap(),
        ),
        (
            m[4].parse::<usize>().unwrap(),
            m[5].parse::<usize>().unwrap(),
        ),
    );

    assert!(range.0.0 <= range.1.0);
    assert!(range.0.1 <= range.1.1);

    let action = match &m[1] {
        "turn on" => Action::TurnOn,
        "turn off" => Action::TurnOff,
        "toggle" => Action::Toggle,
        _ => panic!("Invalid"),
    };

    return Rule{ action: action, range: range };
}

fn compute_lights_lit(rules: &Vec<Rule>, size: usize) -> usize {
    let mut lit_count = 0;
    for x in 0..(size) {
        for y in 0..(size) {
            let lit = rules.iter()
                .filter(|r| r.range.contains((x,y)))
                .fold(false, |state, rule| match rule.action {
                    Action::TurnOff => false,
                    Action::TurnOn => true,
                    Action::Toggle => !state,
                });
            if lit {
                lit_count += 1;
            }
        }
    }
    lit_count
}

fn compute_lights_brightness(rules: &Vec<Rule>, size: usize) -> u64 {
    let mut total_bri: u64 = 0;
    for x in 0..(size) {
        for y in 0..(size) {
            total_bri += rules.iter()
                .filter(|r| r.range.contains((x,y)))
                .fold(0, |state, rule| match rule.action {
                    Action::TurnOff => if state > 0 { state - 1 } else { 0 },
                    Action::TurnOn => state + 1,
                    Action::Toggle => state + 2
                });
        }
    }
    total_bri
}

fn main() {
    let rules: Vec<Rule> = include_str!("../in.txt").split('\n').map(|line|parse_action(line)).collect();
    println!("Part 1: LIT {}", compute_lights_lit(&rules, 1000));
    println!("Part 2: BRI {}", compute_lights_brightness(&rules, 1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_action() {
        assert_eq!(
            parse_action("turn on 0,0 through 999,999"),
            Rule{action:Action::TurnOn,range:Range((0, 0), (999, 999))}
        );
        assert_eq!(
            parse_action("turn on 1,2 through 5,6"),
            Rule{action:Action::TurnOn,range:Range((1, 2), (5, 6))}
        );
        assert_eq!(
            parse_action("toggle 1,2 through 5,6"),
            Rule{action:Action::Toggle,range:Range((1, 2), (5, 6))}
        );
    }

    #[test]
    fn range_contains() {
        assert_eq!(Range((0,0), (1,1)).contains((0,0)), true);
        assert_eq!(Range((0,0), (1,1)).contains((1,0)), true);
        assert_eq!(Range((0,0), (1,1)).contains((1,1)), true);
        assert_eq!(Range((0,0), (1,1)).contains((0,1)), true);
        assert_eq!(Range((0,0), (1,1)).contains((2,0)), false);
        assert_eq!(Range((0,0), (1,1)).contains((2,2)), false);
        assert_eq!(Range((0,0), (1,1)).contains((0,2)), false);
    }
}
