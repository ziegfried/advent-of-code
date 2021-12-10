fn open_to_close(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

fn check_corrupt(line: &str) -> Option<char> {
    let mut stack = vec![];
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            closing => {
                if let Some(last_open) = stack.pop() {
                    if open_to_close(last_open) != closing {
                        return Some(closing);
                    }
                } else {
                    panic!("unexpected empty stack");
                }
            }
        }
    }
    None
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(check_corrupt)
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!(),
        })
        .sum()
}

fn incomplete_score(line: &str) -> Option<usize> {
    let mut stack = vec![];
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            closing => {
                if let Some(last_open) = stack.pop() {
                    if open_to_close(last_open) != closing {
                        return None;
                    }
                } else {
                    panic!("unexpected empty stack");
                }
            }
        }
    }

    Some(
        stack
            .iter()
            .map(|&c| open_to_close(c))
            .map(|c| match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(),
            })
            .rev()
            .fold(0, |score, cur| score * 5 + cur),
    )
}

fn part2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(incomplete_score)
        .collect::<Vec<_>>();
    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
}
fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 26397);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 288957);
}
