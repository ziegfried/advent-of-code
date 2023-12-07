// Problem: https://adventofcode.com/2023/day/7
use std::{cmp::Ordering, collections::HashMap};

type Result = usize;

#[derive(Debug, Clone)]
struct Hand(Vec<usize>);

type Input = Vec<(Vec<usize>, usize)>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            (
                cards
                    .chars()
                    .map(|ch| match ch {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => ch.to_digit(10).unwrap() as usize,
                    })
                    .collect::<Vec<usize>>(),
                bid.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

// ------------------------------------------

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn hand_type(hand: &[usize]) -> HandType {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for i in hand {
        *counts.entry(*i).or_default() += 1;
    }
    if counts.values().any(|count| *count == 5) {
        HandType::FiveOfAKind
    } else if counts.values().any(|count| *count == 4) {
        HandType::FourOfAKind
    } else if counts.values().any(|count| *count == 3) && counts.values().any(|count| *count == 2) {
        HandType::FullHouse
    } else if counts.values().any(|count| *count == 3) {
        HandType::ThreeOfAKind
    } else if counts.values().filter(|count| *count == &2).count() == 2 {
        HandType::TwoPair
    } else if counts.values().any(|count| *count == 2) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn type_rank(hand_type: HandType) -> usize {
    match hand_type {
        HandType::FiveOfAKind => 6,
        HandType::FourOfAKind => 5,
        HandType::FullHouse => 4,
        HandType::ThreeOfAKind => 3,
        HandType::TwoPair => 2,
        HandType::OnePair => 1,
        HandType::HighCard => 0,
    }
}

fn compare_hands(a: &[usize], b: &[usize]) -> Ordering {
    let a_rank = type_rank(hand_type(a));
    let b_rank = type_rank(hand_type(b));
    match a_rank.cmp(&b_rank) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            for i in 0..5 {
                #[allow(clippy::comparison_chain)]
                if a[i] > b[i] {
                    return Ordering::Greater;
                } else if a[i] < b[i] {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}

fn part1(input: &Input) -> Result {
    let mut hands: Vec<_> = input.to_vec();
    hands.sort_by(|(a, _), (b, _)| compare_hands(a, b));
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 6440);
}

// ------------------------------------------

fn has_joker(hand: &[usize]) -> bool {
    hand.iter().any(|c| *c == 0)
}

fn best_variant(hand: &[usize]) -> Vec<usize> {
    (1..=14)
        .map(|repl| {
            hand.iter()
                .map(|c| if *c == 0 { repl } else { *c })
                .collect::<Vec<usize>>()
        })
        .max_by(|a, b| compare_hands(a, b))
        .unwrap()
        .clone()
}

fn part2(input: &Input) -> Result {
    let mut hands: Vec<_> = input
        .iter()
        .map(|(hand, bid)| {
            (
                hand.iter()
                    .map(|v| if *v == 11 { 0 } else { *v })
                    .collect::<Vec<usize>>(),
                *bid,
            )
        })
        .map(|(hand, bid)| {
            (
                if has_joker(&hand) {
                    best_variant(&hand)
                } else {
                    hand.clone()
                },
                hand.clone(),
                bid,
            )
        })
        .collect();

    hands.sort_by(|(a, oa, _), (b, ob, _)| {
        let a_rank = type_rank(hand_type(a));
        let b_rank = type_rank(hand_type(b));
        match a_rank.cmp(&b_rank) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for i in 0..5 {
                    #[allow(clippy::comparison_chain)]
                    if oa[i] > ob[i] {
                        return std::cmp::Ordering::Greater;
                    } else if oa[i] < ob[i] {
                        return std::cmp::Ordering::Less;
                    }
                }
                Ordering::Equal
            }
        }
    });

    hands
        .iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i + 1))
        .sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 5905);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
