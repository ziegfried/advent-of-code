use std::collections::{HashMap, VecDeque};

type Result = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipFlopState(bool);
impl FlipFlopState {
    fn recv(&mut self, high: bool) -> Option<bool> {
        if !high {
            self.0 = !self.0;
            Some(self.0)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConjunctionState(HashMap<String, bool>);
impl ConjunctionState {
    fn add_input(&mut self, from: &str) {
        self.0.insert(from.to_owned(), false);
    }
    fn recv(&mut self, from: &str, high: bool) -> bool {
        self.0.insert(from.to_owned(), high);
        !self.0.values().all(|v| *v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(FlipFlopState, Vec<String>),
    Conjunction(ConjunctionState, Vec<String>),
    Broadcaster(Vec<String>),
}

impl Module {
    fn receive_pulse(&mut self, from: &str, high: bool) -> Vec<(String, bool)> {
        match self {
            Module::FlipFlop(state, seq) => {
                if let Some(next_high) = state.recv(high) {
                    seq.iter().map(|v| (v.clone(), next_high)).collect()
                } else {
                    vec![]
                }
            }
            Module::Conjunction(state, seq) => {
                let next_high = state.recv(from, high);
                seq.iter().map(|v| (v.clone(), next_high)).collect()
            }
            Module::Broadcaster(seq) => seq.iter().map(|v| (v.clone(), false)).collect(),
        }
    }
}

type Input = HashMap<String, Module>;

fn parse_mod(s: &str) -> (String, Module) {
    let (label, seq) = s.split_once(" -> ").unwrap();

    let seq: Vec<String> = seq
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let (t, label) = if let Some(label) = label.strip_prefix('%') {
        (Module::FlipFlop(FlipFlopState(false), seq), label)
    } else if let Some(label) = label.strip_prefix('&') {
        (
            Module::Conjunction(ConjunctionState(HashMap::new()), seq),
            label,
        )
    } else {
        (Module::Broadcaster(seq), label)
    };

    (label.to_string(), t)
}

fn parse_input(input: &str) -> Input {
    let input = input.lines().map(parse_mod).collect::<Input>();
    let mut map = input.clone();
    for (name, value) in map.iter_mut() {
        if let Module::Conjunction(state, _) = value {
            for (parent, other) in input.iter() {
                let seq = match other {
                    Module::FlipFlop(_, seq) => seq,
                    Module::Conjunction(_, seq) => seq,
                    Module::Broadcaster(seq) => seq,
                };
                if seq.contains(name) {
                    state.add_input(parent);
                }
            }
        }
    }
    map
}

// ------------------------------------------

fn count_pulses(state: &mut Input) -> (usize, usize) {
    let mut low_count = 0;
    let mut high_count = 0;
    let mut queue = VecDeque::new();
    queue.push_back(("broadcaster".to_string(), "broadcaster".to_string(), false));
    while let Some((from, name, high)) = queue.pop_front() {
        if high {
            high_count += 1;
        } else {
            low_count += 1;
        }
        if let Some(module) = state.get_mut(&name) {
            let next = module.receive_pulse(&from, high);
            for (n, nh) in next {
                queue.push_back((name.clone(), n.clone(), nh));
            }
        }
    }
    (high_count, low_count)
}

fn part1(input: &Input) -> usize {
    let mut state = input.clone();
    println!();
    let (h, l) = (0..1000).fold((0, 0), |(prev_h, prev_l), _| {
        let (h, l) = count_pulses(&mut state);
        (prev_h + h, prev_l + l)
    });
    h * l
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), (4000 * 8000));
    let input = parse_input(include_str!("test2.txt"));
    assert_eq!(part1(&input), (2750 * 4250));
}

// ------------------------------------------

fn find_parent_conjunction(name: &impl AsRef<str>, input: &Input) -> String {
    let Some(entry) = input.iter().find(|(_, v)| {
        if let Module::Conjunction(_, seq) = v {
            seq == &vec![name.as_ref().to_string()]
        } else {
            false
        }
    }) else {
        panic!()
    };
    entry.0.clone()
}

fn find_high_flip(target: &impl AsRef<str>, input: &Input) -> usize {
    let mut button_presses = 0;
    let mut state = input.clone();
    loop {
        button_presses += 1;
        let mut queue = VecDeque::new();
        queue.push_back(("broadcaster".to_string(), "broadcaster".to_string(), false));
        while let Some((from, name, high)) = queue.pop_front() {
            if let Some(module) = state.get_mut(&name) {
                let next = module.receive_pulse(&from, high);
                if name == target.as_ref() && next[0].1 {
                    return button_presses;
                }
                for (n, nh) in next {
                    queue.push_back((name.clone(), n.clone(), nh));
                }
            }
        }
    }
}

fn part2(input: &Input) -> Result {
    let rx_parent = find_parent_conjunction(&"rx", input);
    let Some(Module::Conjunction(pstate, _)) = input.get(&rx_parent) else {
        panic!()
    };
    pstate
        .0
        .keys()
        .map(|k| find_high_flip(k, input))
        .reduce(num::integer::lcm)
        .unwrap()
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
