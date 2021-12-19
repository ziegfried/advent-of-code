use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let parent_map = input
        .split('\n')
        .map(|line| {
            let child_parent = line.split(')').collect::<Vec<_>>();
            (child_parent[1], child_parent[0])
        })
        .collect::<HashMap<_, _>>();
    let mut count = 0;
    for parent in parent_map.keys() {
        let mut cur_parent = parent;
        while let Some(parent) = parent_map.get(cur_parent) {
            count += 1;
            cur_parent = parent;
        }
    }
    count
}

fn path_to_root(name: &str, parent_map: &HashMap<&str, &str>) -> Vec<String> {
    let mut result = vec![];
    let mut cur = name;
    while let Some(next) = parent_map.get(cur) {
        result.push(String::from(next.clone()));
        cur = next;
    }
    result
}

fn part2(input: &str) -> usize {
    let parent_map = input
        .split('\n')
        .map(|line| {
            let edge_data = line.split(')').collect::<Vec<_>>();
            (edge_data[1], edge_data[0])
        })
        .collect::<HashMap<_, _>>();
    let you_path = path_to_root("YOU", &parent_map);
    let san_path = path_to_root("SAN", &parent_map);
    let ch = you_path
        .iter()
        .find(|c| san_path.contains(&c))
        .clone()
        .unwrap();
    you_path.iter().position(|c| c == ch).unwrap() + san_path.iter().position(|c| c == ch).unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test1.txt")), 42);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test2.txt")), 4);
}
