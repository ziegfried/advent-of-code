use itertools::Itertools;
use std::collections::HashSet;

struct Image {
    pub points: HashSet<(i64, i64)>,
    pub inverted: bool,
}
impl Image {
    fn new(inverted: bool) -> Self {
        Image {
            points: HashSet::new(),
            inverted,
        }
    }
    fn is_lit(&self, pos: &(i64, i64)) -> bool {
        self.points.contains(pos) != self.inverted
    }
    fn set_lit(&mut self, pos: (i64, i64), lit: bool) {
        if lit != self.inverted {
            self.points.insert(pos);
        }
    }
}

fn full_range(image: &Image) -> (i64, i64, i64, i64) {
    let (min_x, max_x) = image
        .points
        .iter()
        .map(|(x, _)| *x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = image
        .points
        .iter()
        .map(|(_, y)| *y)
        .minmax()
        .into_option()
        .unwrap();
    (min_x, max_x, min_y, max_y)
}

fn is_next_pixel_lit(x: i64, y: i64, image: &Image, algo: &[bool]) -> bool {
    let s = ((-1)..=1)
        .map(|dx| {
            ((-1)..=1)
                .map(|dy| match image.is_lit(&(x + dx, y + dy)) {
                    true => '1',
                    false => '0',
                })
                .collect::<String>()
        })
        .collect::<String>();
    let idx = u32::from_str_radix(&s, 2).unwrap() as usize;
    algo[idx]
}

fn enhance(image: &Image, algo: &[bool], invert: bool) -> Image {
    let mut result = Image::new(invert);
    let (min_x, max_x, min_y, max_y) = full_range(image);
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            result.set_lit((x, y), is_next_pixel_lit(x, y, image, algo));
        }
    }
    result
}

fn rasterize(image: &Image, padding: i64) -> String {
    let (min_x, max_x, min_y, max_y) = full_range(image);
    (min_x - padding..=max_x + padding)
        .map(|x| {
            format!(
                "{}\n",
                (min_y - padding..=max_y + padding)
                    .map(|y| match image.is_lit(&(x, y)) {
                        true => '#',
                        false => '.',
                    })
                    .collect::<String>()
            )
        })
        .collect::<String>()
}

const PRINT_ENABLED: bool = false;

fn print(image: &Image) {
    if PRINT_ENABLED {
        println!("{}", rasterize(image, 5));
    }
}

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
    let mut image = Image::new(false);
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            image.set_lit((x as i64, y as i64), grid[x][y] == '#');
        }
    }
    let should_invert = algo[0];
    let mut p1 = 0;
    for step in 0..50 {
        let invert = should_invert && step % 2 == 0;
        print(&image);
        image = enhance(&image, &algo, invert);
        if step == 1 {
            p1 = image.points.len();
        }
    }
    print(&image);
    (p1, image.points.len())
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
