fn solve(input: &str, seq_len: usize) -> usize {
    let input: Vec<char> = input.chars().collect();
    'outer: for i in 0..(input.len() - seq_len) {
        for j in 0..seq_len {
            for k in 0..j {
                if input[i + j] == input[i + k] {
                    continue 'outer;
                }
            }
        }
        return i + seq_len;
    }
    panic!()
}

fn main() {
    println!("Part 1: {}", solve(include_str!("in.txt"), 4));
    println!("Part 2: {}", solve(include_str!("in.txt"), 14));
}

#[test]
fn test_part1() {
    assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
}

#[test]
fn test_part2() {
    assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
}
