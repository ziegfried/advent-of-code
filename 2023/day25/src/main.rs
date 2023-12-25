// Problem: https://adventofcode.com/2023/day/25

use std::collections::{HashMap, HashSet};

type Result = usize;

type Input = HashMap<String, Vec<String>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();

            (
                a.to_string(),
                b.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn check_groups(
    connections: &HashSet<(&String, &String)>,
    disconnects: &Vec<(&String, &String)>,
) -> Option<(usize, usize)> {
    let mut cons = connections.clone();
    let mut starts = vec![];
    for d in disconnects {
        cons.remove(d);
        cons.remove(&(d.1, d.0));
        starts.push(d.0);
        starts.push(d.1);
    }

    let mut seen = HashSet::new();

    {
        let mut queue = vec![starts[0]];
        while let Some(cur) = queue.pop() {
            if !seen.contains(cur) {
                seen.insert(cur);
                for (_, other) in cons.iter().filter(|(a, _)| *a == cur) {
                    queue.push(other);
                }
            }
        }
    }

    let mut seen2 = HashSet::new();

    for &s in starts[1..].iter() {
        if !seen.contains(s) {
            if seen2.is_empty() {
                {
                    let mut queue = vec![s];
                    while let Some(cur) = queue.pop() {
                        if !seen2.contains(cur) {
                            seen2.insert(cur);
                            for (_, other) in cons.iter().filter(|(a, _)| *a == cur) {
                                queue.push(other);
                            }
                        }
                    }
                }
            } else if !seen2.contains(s) {
                // 3rd group
                return None;
            }
        }
    }

    if seen2.is_empty() {
        return None;
    }

    Some((seen.len(), seen2.len()))
}

fn part1(input: &Input, cut: Vec<(&String, &String)>) -> Result {
    let mut unique = HashSet::new();
    let mut connections = HashSet::new();

    for (a, bs) in input.iter() {
        unique.insert(a);
        for b in bs.iter() {
            unique.insert(b);
            connections.insert((a, b));
            connections.insert((b, a));
        }
    }

    if let Some((a, b)) = check_groups(&connections, &cut) {
        return a * b;
    }

    panic!()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    dbg!(&input);
    assert_eq!(
        part1(
            &input,
            vec![
                (&"hfx".to_string(), &"pzl".to_string()),
                (&"bvb".to_string(), &"cmg".to_string()),
                (&"nvd".to_string(), &"jqt".to_string()),
            ]
        ),
        54
    );
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!(
        "Part 1: {:?}",
        part1(
            &input,
            // connections found by visualizing on https://cosmograph.app/
            vec![
                (&"glz".to_string(), &"mxd".to_string()),
                (&"clb".to_string(), &"brd".to_string()),
                (&"jxd".to_string(), &"bbz".to_string()),
            ]
        )
    );
}
