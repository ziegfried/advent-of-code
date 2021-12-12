use std::collections::HashSet;

fn parse_graph(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split('-').collect::<Vec<_>>();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect::<Vec<_>>()
}

fn walk(graph: &Vec<(String, String)>, el: String, stack: &Vec<String>) -> usize {
    if el.to_lowercase() == el {
        if stack.iter().find(|s| s == &&el).is_some() {
            return 0;
        }
    }
    let mut stack = stack.clone();
    stack.push(el.clone());
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
            paths += walk(graph, route.clone(), &stack.clone());
        }
    }
    paths
}

fn part1(input: &str) -> usize {
    let graph = parse_graph(input);
    walk(&graph, "start".to_string(), &vec![])
}

fn walk2(
    graph: &Vec<(String, String)>,
    el: String,
    stack: &Vec<String>,
    all_paths: &mut HashSet<String>,
    extra: Option<String>,
) {
    if el.to_lowercase() == el {
        let count = stack.iter().filter(|s| s == &&el).count();
        let target_count = if Some(el.clone()) == extra { 2 } else { 1 };
        if count == target_count {
            return;
        }
    }
    let mut stack = stack.clone();
    stack.push(el.clone());
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

    for route in routes {
        if *route == "end".to_string() {
            all_paths.insert(stack.join("-"));
        } else {
            walk2(
                graph,
                route.clone(),
                &stack.clone(),
                all_paths,
                extra.clone(),
            );
        }
    }
}

fn part2(input: &str) -> usize {
    let graph = parse_graph(input);
    let lowers = graph
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .filter(|v| v != &&"start" && v != &&"end")
        .filter(|v| v == &&v.to_lowercase())
        .collect::<HashSet<_>>();
    let mut all_paths = HashSet::<String>::new();
    walk2(&graph, "start".to_string(), &vec![], &mut all_paths, None);
    for l in lowers {
        walk2(
            &graph,
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
