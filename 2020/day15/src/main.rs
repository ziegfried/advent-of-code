use std::collections::HashMap;

const PART1_NTH_NUMBER: usize = 2020;
const PART2_NTH_NUMBER: usize = 30000000;

fn spoken_number(numbers: Vec<usize>, until: usize) -> usize {
    let len = numbers.len();
    let mut last_spoken: HashMap<usize, usize> = HashMap::new();
    let mut last = None;
    let mut spoken = 0;
    for i in 0..until {
        if i < len {
            spoken = *numbers.get(i).unwrap();
        } else {
            spoken = match last_spoken.get(&spoken) {
                Some(next) => i - next,
                None => 0,
            }
        }
        if let Some(l) = last {
            last_spoken.insert(l, i);
        }
        last = Some(spoken);
    }
    spoken
}

fn main() {
    println!(
        "Part 1: {}",
        spoken_number(vec![10, 16, 6, 0, 1, 17], PART1_NTH_NUMBER)
    );
    println!(
        "Part 2: {}",
        spoken_number(vec![10, 16, 6, 0, 1, 17], PART2_NTH_NUMBER)
    );
}

#[test]
fn test_part1() {
    assert_eq!(spoken_number(vec![0, 3, 6], PART1_NTH_NUMBER), 436);
    assert_eq!(spoken_number(vec![1, 3, 2], PART1_NTH_NUMBER), 1);
    assert_eq!(spoken_number(vec![2, 1, 3], PART1_NTH_NUMBER), 10);
    assert_eq!(spoken_number(vec![1, 2, 3], PART1_NTH_NUMBER), 27);
    assert_eq!(spoken_number(vec![2, 3, 1], PART1_NTH_NUMBER), 78);
    assert_eq!(spoken_number(vec![3, 2, 1], PART1_NTH_NUMBER), 438);
    assert_eq!(spoken_number(vec![3, 1, 2], PART1_NTH_NUMBER), 1836);
}

#[test]
fn test_part2() {
    assert_eq!(spoken_number(vec![0, 3, 6], PART2_NTH_NUMBER), 175594);
    assert_eq!(spoken_number(vec![1, 3, 2], PART2_NTH_NUMBER), 2578);
    assert_eq!(spoken_number(vec![2, 1, 3], PART2_NTH_NUMBER), 3544142);
    assert_eq!(spoken_number(vec![1, 2, 3], PART2_NTH_NUMBER), 261214);
    assert_eq!(spoken_number(vec![2, 3, 1], PART2_NTH_NUMBER), 6895259);
    assert_eq!(spoken_number(vec![3, 2, 1], PART2_NTH_NUMBER), 18);
    assert_eq!(spoken_number(vec![3, 1, 2], PART2_NTH_NUMBER), 362);
}
