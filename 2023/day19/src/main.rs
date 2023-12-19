// Problem: https://adventofcode.com/2023/day/19

use sscanf::sscanf;
use std::collections::HashMap;

type Result = usize;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Var {
    X,
    M,
    A,
    S,
}
impl Var {
    fn from_str(s: &impl AsRef<str>) -> Self {
        match s.as_ref() {
            "x" => Var::X,
            "m" => Var::M,
            "a" => Var::A,
            "s" => Var::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Cmp {
    Gt,
    Lt,
}
impl Cmp {
    fn compare(&self, a: i32, b: i32) -> bool {
        match self {
            Cmp::Gt => a > b,
            Cmp::Lt => a < b,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Rule(Var, Cmp, i32, String);
impl Rule {
    fn from_str(s: &impl AsRef<str>) -> Self {
        if let Ok((var, val, dest)) = sscanf!(s.as_ref(), "{String}>{i32}:{String}") {
            return Rule(Var::from_str(&var), Cmp::Gt, val, dest);
        }
        if let Ok((var, val, dest)) = sscanf!(s.as_ref(), "{String}<{i32}:{String}") {
            return Rule(Var::from_str(&var), Cmp::Lt, val, dest);
        }
        panic!("{}", s.as_ref())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Ruleset(String, Vec<Rule>, String);

impl Ruleset {
    fn from_str(s: &str) -> Ruleset {
        let (invar, rules) = sscanf!(s, "{String}{{{String}}}").unwrap();
        let rules = rules.split(',').collect::<Vec<_>>();
        Ruleset(
            invar,
            rules[0..rules.len() - 1]
                .iter()
                .map(Rule::from_str)
                .collect(),
            rules[rules.len() - 1].to_string(),
        )
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Rating {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}
impl Rating {
    fn from_str(s: &str) -> Self {
        let (x, m, a, s) = sscanf!(s, "{{x={i32},m={i32},a={i32},s={i32}}}").unwrap();
        Rating { x, m, a, s }
    }
    fn get(&self, var: Var) -> i32 {
        match var {
            Var::X => self.x,
            Var::M => self.m,
            Var::A => self.a,
            Var::S => self.s,
        }
    }
}

type Input = (Vec<Ruleset>, Vec<Rating>);

fn parse_input(input: &str) -> Input {
    let (workflow, ratings) = input.trim().split_once("\n\n").unwrap();
    (
        workflow.lines().map(Ruleset::from_str).collect(),
        ratings.lines().map(Rating::from_str).collect(),
    )
}

// ------------------------------------------

fn eval_wf(rule: &String, rt: &Rating, rule_map: &HashMap<String, Ruleset>) -> bool {
    let Ruleset(_, rules, fallback) = rule_map.get(rule).unwrap();
    for Rule(var, cmp, cmp_val, dest) in rules {
        if cmp.compare(rt.get(*var), *cmp_val) {
            if dest == "A" {
                return true;
            }
            if dest == "R" {
                return false;
            }
            return eval_wf(dest, rt, rule_map);
        }
    }
    if fallback == "A" {
        return true;
    }
    if fallback == "R" {
        return false;
    }
    eval_wf(fallback, rt, rule_map)
}

fn part1((workflow, ratings): &Input) -> Result {
    let rule_map: HashMap<String, Ruleset> = workflow
        .iter()
        .map(|rs| (rs.0.clone(), rs.clone()))
        .collect();

    let mut total = 0;
    for rt in ratings {
        if eval_wf(&"in".to_string(), rt, &rule_map) {
            total += rt.x + rt.m + rt.a + rt.s;
        }
    }
    total as usize
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 19114);
}

// ------------------------------------------

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct PossibleRange(Vec<bool>);
impl PossibleRange {
    fn new() -> Self {
        Self(vec![true; 4000])
    }
    fn constrain(&mut self, cmp: Cmp, val: i32, inverse: bool) {
        for v in 0..4000 {
            if cmp.compare(v as i32 + 1, val) == inverse {
                self.0[v] = false;
            }
        }
    }
    fn is_possible(&self) -> bool {
        self.0.iter().any(|v| *v)
    }
    fn possible_values(&self) -> usize {
        if self.is_possible() {
            self.0.iter().filter(|v| **v).count()
        } else {
            0
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    step: String,
    x: PossibleRange,
    m: PossibleRange,
    a: PossibleRange,
    s: PossibleRange,
}
impl State {
    fn start() -> Self {
        State {
            step: "in".to_string(),
            x: PossibleRange::new(),
            m: PossibleRange::new(),
            a: PossibleRange::new(),
            s: PossibleRange::new(),
        }
    }
    fn constrain(&self, var: Var, cmp: Cmp, val: i32, inverse: bool, next_step: String) -> State {
        let mut new_state = self.clone();
        match var {
            Var::X => {
                new_state.x.constrain(cmp, val, inverse);
            }
            Var::M => {
                new_state.m.constrain(cmp, val, inverse);
            }
            Var::A => {
                new_state.a.constrain(cmp, val, inverse);
            }
            Var::S => {
                new_state.s.constrain(cmp, val, inverse);
            }
        };
        new_state.step = next_step;
        new_state
    }
    fn possible_values(&self) -> usize {
        self.x.possible_values()
            * self.m.possible_values()
            * self.a.possible_values()
            * self.s.possible_values()
    }
    fn is_possible(&self) -> bool {
        self.x.is_possible() && self.m.is_possible() && self.a.is_possible() && self.s.is_possible()
    }
}

fn possible_ratings(state: State, rule_map: &HashMap<String, Ruleset>) -> usize {
    if state.step == "R" || !state.is_possible() {
        return 0;
    }
    if state.step == "A" {
        return state.possible_values();
    }
    let mut total = 0;
    let mut state = state.clone();
    let Ruleset(_, rules, fallback) = rule_map[&state.step].clone();
    for Rule(var, cmp, val, next) in rules {
        total += possible_ratings(state.constrain(var, cmp, val, false, next), rule_map);
        state = state.constrain(var, cmp, val, true, fallback.clone());
    }
    total + possible_ratings(state, rule_map)
}

fn part2((workflow, _): &Input) -> Result {
    let rule_map: HashMap<String, Ruleset> = workflow
        .iter()
        .map(|rs| (rs.0.clone(), rs.clone()))
        .collect();

    possible_ratings(State::start(), &rule_map)
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 167409079868000);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
