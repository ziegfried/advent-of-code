use petgraph::algo::floyd_warshall;
use petgraph::{Directed, Graph};
use std::collections::{HashMap, VecDeque};

fn parse_input(input: &str) -> Vec<(String, usize, Vec<String>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            match sscanf::sscanf!(
                line,
                "Valve {String} has flow rate={usize}; {String} {String} to {String} {String}"
            ) {
                Ok(r) => r,
                Err(_) => panic!("unable to match: {}", line),
            }
        })
        .map(|(origin, rate, _, _, _, destinations)| {
            (
                origin,
                rate,
                destinations
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect()
}

fn calc_distances(input: &[(String, usize, Vec<String>)]) -> HashMap<(String, String), usize> {
    let mut graph: Graph<String, (), Directed> = Graph::new();
    let graph_nodes = input
        .iter()
        .map(|(name, _, _)| (name.clone(), graph.add_node(name.clone())))
        .collect::<HashMap<_, _>>();
    for (id, _, next) in input.iter() {
        for dest in next.iter() {
            graph.add_edge(graph_nodes[id], graph_nodes[dest], ());
        }
    }
    floyd_warshall(&graph, |_| 1)
        .unwrap()
        .iter()
        .map(|((from, to), cost)| {
            (
                (
                    graph.node_weight(*from).unwrap().clone(),
                    graph.node_weight(*to).unwrap().clone(),
                ),
                *cost as usize,
            )
        })
        .collect()
}

fn extend_vec<T: Clone>(vec: &Vec<T>, el: T) -> Vec<T> {
    let mut result = Vec::with_capacity(vec.len() + 1);
    for it in vec {
        result.push(it.clone());
    };
    result.push(el);
    result
}

fn part1(input: &str) -> usize {
    let input = parse_input(input);
    let distances = calc_distances(&input);
    let nodes_with_rate = input
        .iter()
        .filter(|(_, r, _)| r > &0)
        .map(|(origin, rate, _)| (origin.clone(), *rate))
        .collect::<HashMap<String, usize>>();

    let minutes = 30;
    let mut best = 0;
    let mut queue = VecDeque::new();
    let start = "AA".to_string();
    queue.push_back((&start, 0, vec![], 0));

    while let Some((id, minute, open, released)) = queue.pop_front() {
        if released > best {
            best = released;
        }
        for next in nodes_with_rate.keys() {
            if !open.contains(&next) {
                let dist = distances.get(&(id.clone(), (*next).clone())).unwrap();
                if minute + dist < minutes {
                    let rate = nodes_with_rate.get(next).unwrap();
                    let released = released + rate * (minutes - dist - minute - 1);
                    queue.push_back((next, minute + dist + 1, extend_vec(&open, next), released));
                }
            }
        }
    }

    best
}

fn part2(input: &str) -> usize {
    let input = parse_input(input);
    let distances = calc_distances(&input);
    let nodes_with_rate = input
        .iter()
        .filter(|(_, r, _)| r > &0)
        .map(|(origin, rate, _)| (origin.clone(), *rate))
        .collect::<HashMap<String, usize>>();

    let minutes = 26;
    let mut best = 0;
    let mut queue = VecDeque::new();
    let start = "AA".to_string();
    queue.push_back((&start, &start, 0, 0, vec![], 0));
    let mut best_cache = HashMap::new();

    while let Some((a, b, a_minute, b_minute, open, released)) = queue.pop_front() {
        if released > best {
            best = released;
        }

        let k = (a, b, a_minute, b_minute);
        if let Some(&prev_best) = best_cache.get(&k) {
            if prev_best >= released {
                continue;
            }
        }
        best_cache.insert(k, released);

        let [a_next, b_next] = [(a, a_minute), (b, b_minute)].map(|(node, min)| {
            nodes_with_rate
                .keys()
                .filter(|d| {
                    !open.contains(d)
                        && distances.get(&(node.clone(), (*d).clone())).unwrap() < &(minutes - min)
                })
                .collect::<Vec<_>>()
        });

        for &a_next in a_next.iter() {
            let a_rate = nodes_with_rate.get(a_next).unwrap();
            let a_dist = distances.get(&(a.clone(), (*a_next).clone())).unwrap();
            let a_newly_released = a_rate * (minutes - a_dist - a_minute - 1);
            queue.push_back((
                a_next,
                b,
                a_minute + a_dist + 1,
                b_minute,
                extend_vec(&open, a_next),
                released + a_newly_released,
            ));
        }

        for &b_next in b_next.iter() {
            let b_rate = nodes_with_rate.get(b_next).unwrap();
            let b_dist = distances.get(&(b.clone(), (*b_next).clone())).unwrap();
            let b_newly_released = b_rate * (minutes - b_dist - b_minute - 1);

            queue.push_back((
                a,
                b_next,
                a_minute,
                b_minute + b_dist + 1,
                extend_vec(&open, b_next),
                released + b_newly_released,
            ));
        }
    }

    best
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 1651);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 1707);
}
