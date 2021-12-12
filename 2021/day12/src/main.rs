use std::collections::{HashMap, HashSet};

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let graph = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            (left.to_string(), right.to_string())
        })
        .collect::<Vec<_>>();
    let all_nodes = graph
        .iter()
        .flat_map(|(a, b)| vec![a.clone(), b.clone()])
        .collect::<Vec<String>>();
    all_nodes
        .iter()
        .map(|e| {
            (
                e.clone(),
                graph
                    .iter()
                    .filter_map(|(start, end)| {
                        if start == e {
                            Some(end.clone())
                        } else if end == e {
                            Some(start.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>(),
            )
        })
        .collect()
}

fn is_small_cave(s: &String) -> bool {
    s.chars().next().unwrap().is_lowercase()
}

fn walk(
    graph: &HashMap<String, Vec<String>>,
    el: String,
    cur_path: &HashSet<String>,
    small_extra: bool,
) -> usize {
    let mut small_extra = small_extra;
    if is_small_cave(&el) {
        if cur_path.contains(&el) {
            if small_extra && el != "end" && el != "start" {
                small_extra = false;
            } else {
                return 0;
            }
        }
    }
    let mut cur_path = cur_path.clone();
    cur_path.insert(el.clone());
    let mut paths = 0;
    if let Some(routes) = graph.get(&el) {
        for route in routes {
            if *route == "end".to_string() {
                paths += 1;
            } else {
                paths += walk(graph, route.clone(), &cur_path.clone(), small_extra);
            }
        }
    }
    paths
}

fn part1(input: &str) -> usize {
    let graph = parse_graph(input);
    walk(&graph, "start".to_string(), &HashSet::new(), false)
}

fn part2(input: &str) -> usize {
    let graph = parse_graph(input);
    walk(&graph, "start".to_string(), &HashSet::new(), true)
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 10);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 36);
}
