fn fuel_required(mass: usize) -> usize {
    mass / 3 - 2
}

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mass| fuel_required(mass))
        .sum()
}

fn fuel_required_recursive(mass: isize) -> isize {
    let req = mass / 3 - 2;
    if req > 0 {
        fuel_required_recursive(req) + req
    } else {
        0
    }
}

fn part2(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| line.parse::<isize>().unwrap())
        .map(|mass| fuel_required_recursive(mass))
        .sum::<isize>() as usize
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("in.txt")));
    println!("Part 2: {:?}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(fuel_required(12), 2);
    assert_eq!(fuel_required(14), 2);
    assert_eq!(fuel_required(1969), 654);
    assert_eq!(part1(include_str!("test.txt")), 2 + 2 + 654 + 33583);
}

#[test]
fn test_part2() {
    assert_eq!(fuel_required_recursive(14), 2);
    assert_eq!(fuel_required_recursive(1969), 966);
    assert_eq!(fuel_required_recursive(100756), 50346);
    assert_eq!(part2(include_str!("test.txt")), 2 + 2 + 966 + 50346);
}
