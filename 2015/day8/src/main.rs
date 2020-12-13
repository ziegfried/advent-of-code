fn unescaped_length(s: &str) -> usize {
    let mut it = s.chars();
    let mut len = 0;

    match it.next().unwrap() {
        '"' => {}
        _ => panic!("open quote missing"),
    }

    loop {
        match it.next().unwrap() {
            '\\' => match it.next().unwrap() {
                '"' => {
                    len += 1;
                }
                '\\' => {
                    len += 1;
                }
                'x' => {
                    it.next().unwrap();
                    it.next().unwrap();
                    len += 1;
                }
                _ => panic!("invalid escape sequence"),
            },
            '"' => {
                if it.next().is_some() {
                    panic!("expected end after quote");
                }
                break;
            }
            _ => {
                len += 1;
            }
        }
    }

    len
}

fn escaped_length(s: &str) -> usize {
    let chars: usize = s
        .chars()
        .map(|c| match c {
            '"' => 2,
            '\\' => 2,
            _ => 1,
        })
        .sum();
    chars + 2
}

fn main() {
    let part1: usize = include_str!("../in.txt")
        .split('\n')
        .map(|line| line.len() - unescaped_length(line))
        .sum();
    println!("Part 1: {}", part1);
    let part2: usize = include_str!("../in.txt")
        .split('\n')
        .map(|line| escaped_length(line) - line.len())
        .sum();
    println!("Part 2: {}", part2);
}
