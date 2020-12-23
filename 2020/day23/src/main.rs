fn serialize_linked_list(links: &Vec<usize>, dest: &mut [usize], start: usize) {
    let mut p = start;
    for i in 0..dest.len() {
        dest[i] = p;
        p = links[p];
    }
}

fn run_game(cups: &mut [usize], moves: usize) {
    let len = cups.len();
    let max = cups.iter().max().unwrap();
    let mut links = vec![0; len + 1];
    for pair in cups.windows(2) {
        links[pair[0]] = pair[1];
    }
    links[cups[len - 1]] = cups[0];
    let mut cur = cups[0];
    for _ in 0..moves {
        let a = links[cur];
        let b = links[a];
        let c = links[b];
        let picked = vec![a, b, c];
        let after = links[c];
        links[cur] = after;
        let mut destination = None;
        for dest in (0..cur).rev().chain((0..=*max).rev()) {
            if links[dest] != 0 && !&picked.contains(&&&dest) {
                destination = Some(dest);
                break;
            }
        }
        let destination = destination.unwrap();
        let rest = links[destination];
        links[destination] = a;
        links[c] = rest;
        cur = links[cur];
    }
    serialize_linked_list(&links, cups, 1);
}

fn part1(s: &str, moves: usize) -> usize {
    let mut cups = s
        .chars()
        .map(|c| String::from(c).parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    run_game(&mut cups, moves);
    while cups[0] != 1 {
        cups.rotate_left(1);
    }
    cups.iter()
        .skip(1)
        .map(|c| format!("{}", c))
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn part2(s: &str) -> usize {
    let mut cups = Vec::with_capacity(1000000);
    for i in s.chars().map(|c| String::from(c).parse::<usize>().unwrap()) {
        cups.push(i);
    }
    for cup in 10..=1000000 {
        cups.push(cup);
    }
    run_game(&mut cups, 10_000_000);
    cups[1] * cups[2]
}

fn main() {
    println!("Part 1: {}", part1(&"394618527", 100));
    println!("Part 2: {}", part2(&"394618527"));
}

#[test]
fn test_part1() {
    assert_eq!(part1(&"389125467", 10), 92658374);
    assert_eq!(part1(&"389125467", 100), 67384529);
    assert_eq!(part1(&"389125467", 100), 67384529);
    assert_eq!(part1(&"394618527", 100), 78569234);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&"389125467"), 149245887792);
}
