use std::cmp::min;

fn count_combinations(containers: Vec<usize>, target: usize) -> usize {
    let mut sorted = containers.clone();
    sorted.sort();
    sorted.reverse();
    fn combos(sorted_containers: &[usize], remaining: usize) -> usize {
        match (0..sorted_containers.len()).find(|i| sorted_containers[*i] <= remaining) {
            None => 0,
            Some(i) => {
                let v = sorted_containers[i];
                if v == remaining {
                    1 + combos(&sorted_containers[(i + 1)..], remaining)
                } else {
                    combos(&sorted_containers[(i + 1)..], remaining - v)
                        + combos(&sorted_containers[(i + 1)..], remaining)
                }
            }
        }
    }
    combos(&sorted[..], target)
}

fn min_container_count(containers: Vec<usize>, target: usize) -> usize {
    let mut sorted = containers.clone();
    sorted.sort();
    sorted.reverse();
    fn min_combos(sorted_containers: &[usize], remaining: usize, count: usize) -> usize {
        match (0..sorted_containers.len()).find(|i| sorted_containers[*i] <= remaining) {
            None => usize::MAX,
            Some(i) => {
                let v = sorted_containers[i];
                if v == remaining {
                    min(
                        count + 1,
                        min_combos(&sorted_containers[(i + 1)..], remaining, count),
                    )
                } else {
                    min(
                        min_combos(&sorted_containers[(i + 1)..], remaining - v, count + 1),
                        min_combos(&sorted_containers[(i + 1)..], remaining, count),
                    )
                }
            }
        }
    }
    min_combos(&sorted[..], target, 0)
}

fn count_combinations_with_count(containers: Vec<usize>, target: usize, count: usize) -> usize {
    let mut sorted = containers.clone();
    sorted.sort();
    sorted.reverse();
    fn combos(sorted_containers: &[usize], remaining: usize, count: usize) -> usize {
        match (0..sorted_containers.len()).find(|i| sorted_containers[*i] <= remaining) {
            None => 0,
            Some(i) => {
                let v = sorted_containers[i];
                if v == remaining && count == 1 {
                    1 + combos(&sorted_containers[(i + 1)..], remaining, count)
                } else {
                    combos(&sorted_containers[(i + 1)..], remaining - v, count - 1)
                        + combos(&sorted_containers[(i + 1)..], remaining, count)
                }
            }
        }
    }
    combos(&sorted[..], target, count)
}

fn main() {
    let containers = include_str!("in.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("Part 1: {}", count_combinations(containers.clone(), 150));

    let min_count = min_container_count(containers.clone(), 150);
    dbg!(min_count);
    println!(
        "Part 2: {}",
        count_combinations_with_count(containers.clone(), 150, min_count)
    );
}

#[test]
fn test_part1() {
    assert_eq!(count_combinations(vec![20, 15, 10, 5, 5], 25), 4);
}

#[test]
fn test_part2() {
    assert_eq!(min_container_count(vec![20, 15, 10, 5, 5], 25), 2);
}
