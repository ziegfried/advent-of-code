use std::collections::{HashMap, HashSet};

fn parse_graph(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split('-').collect::<Vec<_>>();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect::<Vec<_>>()
}

fn walk(graph: &Vec<(String, String)>, el: String, path: &Vec<String>) -> usize {
    if el.to_lowercase() == el {
        if path.iter().find(|s| s == &&el).is_some() {
            return 0;
        }
    }
    let mut path = path.clone();
    path.push(el.clone());
    let routes = graph
        .iter()
        .filter_map(|(start, end)| {
            if start == &el {
                Some(end)
            } else if end == &el {
                Some(start)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut paths = 0;
    for route in routes {
        if *route == "end".to_string() {
            paths += 1;
        } else {
            paths += walk(graph, route.clone(), &path.clone());
        }
    }
    paths
}

fn part1(input: &str) -> usize {
    let graph = parse_graph(input);
    walk(&graph, "start".to_string(), &vec![])
}

fn walk2(
    graph_map: &HashMap<String, Vec<String>>,
    el: String,
    path: &Vec<String>,
    all_paths: &mut HashSet<String>,
    extra: Option<String>,
) {
    if el.to_lowercase() == el {
        let count = path.iter().filter(|s| s == &&el).count();
        let target_count = if Some(el.clone()) == extra { 2 } else { 1 };
        if count == target_count {
            return;
        }
    }
    let mut path = path.clone();
    path.push(el.clone());
    for next_route in graph_map.get(&el).unwrap() {
        if *next_route == "end".to_string() {
            all_paths.insert(path.join("-"));
        } else {
            walk2(
                graph_map,
                next_route.clone(),
                &path,
                all_paths,
                extra.clone(),
            );
        }
    }
}

fn part2(input: &str) -> usize {
    let graph = parse_graph(input);
    let all_nodes = graph
        .iter()
        .flat_map(|(a, b)| vec![a.clone(), b.clone()])
        .collect::<Vec<String>>();
    let graph_map: HashMap<String, Vec<String>> = all_nodes
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
        .collect();
    let lower_nodes = all_nodes
        .iter()
        .filter(|v| v != &&"start" && v != &&"end")
        .filter(|v| v == &&v.to_lowercase())
        .collect::<HashSet<_>>();
    let mut all_paths = HashSet::<String>::new();
    walk2(
        &graph_map,
        "start".to_string(),
        &vec![],
        &mut all_paths,
        None,
    );
    for l in lower_nodes {
        walk2(
            &graph_map,
            "start".to_string(),
            &vec![],
            &mut all_paths,
            Some(l.clone()),
        );
    }
    all_paths.len()
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
