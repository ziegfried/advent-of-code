use std::collections::HashSet;

type Tile = (isize, isize);

#[derive(Debug, Clone)]
enum Dir {
    East,
    West,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast,
}

fn parse_directions(line: &str) -> Vec<Dir> {
    let mut result = vec![];
    let mut it = line.chars();
    loop {
        match it.next() {
            Some(c) => {
                let dir = match c {
                    'e' => Dir::East,
                    'w' => Dir::West,
                    's' => match it.next().unwrap() {
                        'w' => Dir::SouthWest,
                        'e' => Dir::SouthEast,
                        _ => panic!(),
                    },
                    'n' => match it.next().unwrap() {
                        'w' => Dir::NorthWest,
                        'e' => Dir::NorthEast,
                        _ => panic!(),
                    },
                    _ => panic!(),
                };
                result.push(dir);
            }
            None => break,
        }
    }
    result
}

fn move_dir(pos: Tile, dir: Dir) -> Tile {
    use Dir::*;
    let (x, y) = pos;
    match dir {
        East => (x + 1, y),
        West => (x - 1, y),
        SouthEast => (x, y + 1),
        SouthWest => (x - 1, y + 1),
        NorthEast => (x + 1, y - 1),
        NorthWest => (x, y - 1),
    }
}

fn move_path(pos: Tile, dirs: Vec<Dir>) -> Tile {
    dirs.iter().fold(pos, |pos, dir| move_dir(pos, dir.clone()))
}

fn part1(input: &str) -> usize {
    let mut black: HashSet<Tile> = HashSet::new();
    for line in input.lines() {
        let dirs = parse_directions(line);
        let pos = move_path((0, 0), dirs);
        if black.contains(&pos) {
            &black.remove(&pos);
        } else {
            &black.insert(pos);
        }
    }
    black.len()
}

fn neighbors(pos: Tile) -> Vec<Tile> {
    use Dir::*;
    vec![East, West, SouthEast, SouthWest, NorthWest, NorthEast]
        .iter()
        .map(|dir| move_dir(pos, dir.clone()))
        .collect::<Vec<Tile>>()
}

fn part2(input: &str) -> usize {
    let mut black: HashSet<Tile> = HashSet::new();
    for line in input.lines() {
        let pos = move_path((0, 0), parse_directions(line));
        if black.contains(&pos) {
            &black.remove(&pos);
        } else {
            &black.insert(pos);
        }
    }

    for _ in 0..100 {
        let flip_to_white = black
            .iter()
            .cloned()
            .filter(|tile| {
                let black_neighbors = neighbors(tile.clone())
                    .iter()
                    .filter(|n| *&black.contains(n))
                    .count();

                black_neighbors == 0 || black_neighbors > 2
            })
            .collect::<Vec<Tile>>();
        let flip_to_black = black
            .iter()
            .cloned()
            .flat_map(|b| neighbors(b))
            .filter(|p| !black.contains(p))
            .filter(|p| {
                neighbors(p.clone())
                    .iter()
                    .filter(|n| black.contains(n))
                    .count()
                    == 2
            })
            .collect::<Vec<Tile>>();
        for t in flip_to_white.iter() {
            black.remove(t);
        }
        for t in flip_to_black.iter() {
            black.insert(t.clone());
        }
    }
    black.len()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_move_path() {
    assert_eq!(move_path((0, 0), parse_directions("nwwswee")), (0, 0));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 10);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 2208);
}
