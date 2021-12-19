fn part1(input: &str) -> usize {
    let mut fish = input
        .split(',')
        .map(|v| v.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..80 {
        let len = fish.len();
        for i in 0..len {
            let mut v = fish[i];
            if v > 0 {
                v -= 1;
            } else {
                v = 6;
                fish.push(8);
            }
            fish[i] = v
        }
    }
    fish.len()
}

fn part2(input: &str) -> usize {
    let fish = input
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut timers = [0; 9];
    for f in fish {
        timers[f] += 1;
    }
    for _ in 0..256 {
        let reset_count = timers[0];
        for i in 1..=8 {
            timers[i - 1] = timers[i];
        }
        timers[8] = reset_count;
        timers[6] += reset_count;
    }
    timers.iter().sum()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 5934);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 26984457539);
}
