use itertools::Itertools;
use std::collections::HashSet;

fn full_range(image: &HashSet<(i64, i64)>) -> (i64, i64, i64, i64) {
    let (min_x, max_x) = image
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = image
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();
    (min_x, max_x, min_y, max_y)
}

fn is_pixel_lit(
    x: i64,
    y: i64,
    image: &HashSet<(i64, i64)>,
    algo: &Vec<bool>,
    is_inverted: bool,
) -> bool {
    let s = ((-1)..=1)
        .map(|dx| {
            ((-1)..=1)
                .map(
                    |dy| match image.contains(&(x + dx, y + dy)) != is_inverted {
                        true => '1',
                        false => '0',
                    },
                )
                .collect::<String>()
        })
        .collect::<String>();
    let idx = u32::from_str_radix(&s, 2).unwrap() as usize;
    algo[idx]
}

fn enhance(
    image: &HashSet<(i64, i64)>,
    algo: &Vec<bool>,
    invert: bool,
    is_inverted: bool,
) -> HashSet<(i64, i64)> {
    let (min_x, max_x, min_y, max_y) = full_range(&image);
    let mut result: HashSet<(i64, i64)> = HashSet::new();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            if is_pixel_lit(x, y, image, algo, is_inverted) != invert {
                result.insert((x, y));
            }
        }
    }
    result
}

fn rasterize(image: &HashSet<(i64, i64)>) -> String {
    let (min_x, max_x, min_y, max_y) = full_range(&image);
    (min_x - 1..=max_x + 1)
        .map(|x| {
            format!(
                "{}\n",
                (min_y - 1..=max_y + 1)
                    .map(|y| match image.contains(&(x, y)) {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>()
            )
        })
        .collect::<String>()
}

const PRINT: bool = false;

fn solve(input: &str) -> (usize, usize) {
    let (algo, grid) = input.split_once("\n\n").unwrap();
    let algo: Vec<bool> = algo
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!(),
        })
        .collect();
    assert_eq!(algo.len(), 512);

    let grid = grid
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut image = HashSet::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '#' {
                image.insert((x as i64, y as i64));
            }
        }
    }
    let should_invert = algo[0];
    let mut p1 = 0;
    for step in 0..50 {
        let invert = should_invert && step % 2 == 0;
        let is_inverted = should_invert && step % 2 == 1;
        image = enhance(&image, &algo, invert, is_inverted);
        if step == 1 {
            p1 = image.len();
        }
        if PRINT {
            println!("{}\n-------\n", rasterize(&image));
        }
    }
    if PRINT {
        println!("{}", rasterize(&image));
    }
    (p1, image.len())
}

fn main() {
    let (p1, p2) = solve(include_str!("in.txt"));
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[test]
fn test_example() {
    assert_eq!(solve(include_str!("test.txt")), (35, 3351));
}

#[test]
fn test_solution() {
    assert_eq!(solve(include_str!("in.txt")), (5361, 16826));
}
