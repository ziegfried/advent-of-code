// Problem: https://adventofcode.com/2023/day/12

type Result = usize;

type Input = Vec<(Vec<char>, Vec<usize>)>;

fn parse_row(s: &str) -> (Vec<char>, Vec<usize>) {
    let (a, b) = s.split_once(' ').unwrap();
    (
        a.chars().collect::<Vec<char>>(),
        b.split(',').map(|s| s.parse::<usize>().unwrap()).collect(),
    )
}

fn parse_input(input: &str) -> Input {
    input.lines().map(parse_row).collect()
}

fn is_valid(springs: &Vec<char>, groups: &[usize]) -> bool {
    let mut it = groups.iter();
    let mut cur: i32 = -1;
    for s in springs {
        match *s {
            '#' => {
                if cur == -1 {
                    if let Some(next) = it.next() {
                        cur = *next as i32 - 1;
                    } else {
                        return false;
                    }
                } else {
                    cur -= 1;
                }
            }
            '.' => {
                if cur == 0 {
                    cur = -1;
                }
                if cur != -1 {
                    return false;
                }
            }
            _ => {
                panic!()
            }
        }
    }
    (cur == 0 || cur == -1) && it.next().is_none()
}

#[test]
fn test_is_valid() {
    assert!(is_valid(&"##.#".chars().collect(), &[2, 1]));
    assert!(is_valid(&".###.##.#...".chars().collect(), &[3, 2, 1]));
    assert!(!is_valid(&"###.".chars().collect(), &[2, 1]));
    assert!(!is_valid(&"####......##".chars().collect(), &[3, 2, 1]));
}

use std::collections::{
    btree_set::{BTreeSet, IntoIter},
    HashMap,
};

enum UniquePermutations {
    Leaf {
        elements: Option<Vec<char>>,
    },
    Stem {
        elements: Vec<char>,
        unique_elements: IntoIter<char>,
        first_element: char,
        inner: Box<Self>,
    },
}
impl UniquePermutations {
    fn new(elements: Vec<char>) -> Self {
        if elements.len() == 1 {
            let elements = Some(elements);
            Self::Leaf { elements }
        } else {
            let mut unique_elements = elements
                .clone()
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter();
            let (first_element, inner) =
                Self::next_level(&mut unique_elements, elements.clone()).unwrap();
            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            }
        }
    }
    fn next_level(
        mut unique_elements: impl Iterator<Item = char>,
        elements: Vec<char>,
    ) -> Option<(char, Box<Self>)> {
        let first_element = unique_elements.next()?;
        let mut remaining_elements = elements;
        if let Some(idx) = remaining_elements.iter().position(|&i| i == first_element) {
            remaining_elements.remove(idx);
        }
        let inner = Box::new(Self::new(remaining_elements));
        Some((first_element, inner))
    }
}

impl Iterator for UniquePermutations {
    type Item = Vec<char>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Leaf { elements } => elements.take(),
            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            } => loop {
                match inner.next() {
                    Some(mut v) => {
                        v.insert(0, *first_element);
                        return Some(v);
                    }
                    None => {
                        let (next_fe, next_i) =
                            Self::next_level(&mut *unique_elements, elements.clone())?;
                        *first_element = next_fe;
                        *inner = next_i;
                    }
                }
            },
        }
    }
}

fn arrangements(springs: &[char], groups: &[usize]) -> usize {
    let wildcards = springs.iter().filter(|s| **s == '?').count();
    let total_operational: usize = groups.iter().sum();
    let have_operational: usize = springs.iter().filter(|s| **s == '#').count();
    let need_op = total_operational - have_operational;
    let mut a = (0..need_op).map(|_| '#').collect::<Vec<char>>();
    let mut b = (0..(wildcards - need_op))
        .map(|_| '.')
        .collect::<Vec<char>>();
    a.append(&mut b);
    UniquePermutations::new(a)
        .map(|c| {
            let mut it = c.iter();
            springs
                .iter()
                .map(|s| if s == &'?' { *it.next().unwrap() } else { *s })
                .collect::<Vec<char>>()
        })
        .filter(|s| is_valid(s, groups))
        .count()
}

fn part1(input: &Input) -> Result {
    input.iter().map(|(s, g)| arrangements(s, g)).sum()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 10);
}

// ------------------------------------------

fn find_arrangements(
    pos: usize,
    group: usize,
    group_pos: usize,
    springs: &Vec<char>,
    groups: &Vec<usize>,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(res) = cache.get(&(pos, group, group_pos)) {
        return *res;
    }
    if pos < springs.len() {
        let mut res = 0;
        if springs[pos] == '#' || springs[pos] == '?' {
            res += find_arrangements(pos + 1, group, group_pos + 1, springs, groups, cache)
        }
        if springs[pos] == '.' || springs[pos] == '?' {
            if group_pos == 0 {
                res += find_arrangements(pos + 1, group, 0, springs, groups, cache);
            } else if group_pos > 0 && group < groups.len() && groups[group] == group_pos {
                res += find_arrangements(pos + 1, group + 1, 0, springs, groups, cache);
            }
        }
        cache.insert((pos, group, group_pos), res);
        return res;
    }

    if (group == groups.len() - 1 && groups[group] == group_pos)
        || (group == groups.len() && group_pos == 0)
    {
        1
    } else {
        0
    }
}

fn fivex<T: Clone>(list: &Vec<T>, delim: &Vec<T>) -> Vec<T> {
    let mut result = vec![];
    for i in 0..5 {
        for v in list {
            result.push(v.clone());
        }
        if i != 4 {
            for d in delim {
                result.push(d.clone());
            }
        }
    }
    result
}

fn part2(input: &Input) -> Result {
    input
        .iter()
        .map(|(s, g)| (fivex(s, &vec!['?']), fivex(g, &vec![])))
        .map(|(s, g)| find_arrangements(0, 0, 0, &s, &g, &mut HashMap::new()))
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2(&vec![parse_row("???.### 1,1,3")]), 1);
    assert_eq!(part2(&vec![parse_row(".??..??...?##. 1,1,3")]), 16384);
    assert_eq!(part2(&vec![parse_row("?#?#?#?#?#?#?#? 1,3,1,6")]), 1);
    assert_eq!(part2(&vec![parse_row("????.#...#... 4,1,1")]), 16);
    assert_eq!(part2(&vec![parse_row("????.######..#####. 1,6,5")]), 2500);
    assert_eq!(part2(&vec![parse_row("?###???????? 3,2,1")]), 506250);
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
