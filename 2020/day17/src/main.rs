use std::cmp::{max, min};
use std::ops::RangeInclusive;

type Int = isize;
type Cube = (Int, Int, Int);

enum Dir {
    X,
    Y,
    Z,
    W,
}

struct Grid(Vec<Cube>);

impl Grid {
    fn from_str(s: &str) -> Self {
        let grid = s
            .split('\n')
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(y, c)| match c {
                        '#' => Some((x as isize, y as isize, 0 as isize)),
                        _ => None,
                    })
                    .collect::<Vec<Cube>>()
            })
            .collect::<Vec<Cube>>();
        Grid::from_vec(grid)
    }

    fn from_vec(vec: Vec<Cube>) -> Self {
        Grid(vec)
    }

    fn range(&self, dir: Dir) -> RangeInclusive<Int> {
        let range = self
            .0
            .iter()
            .map(|cube| match &dir {
                Dir::X => cube.0,
                Dir::Y => cube.1,
                Dir::Z => cube.2,
                _ => panic!(),
            })
            .fold((0isize, 0isize), |(cur_min, cur_max), cur| {
                (min(cur, cur_min), max(cur, cur_max))
            });
        (range.0)..=(range.1)
    }

    fn extended_range(&self, dir: Dir) -> RangeInclusive<Int> {
        let range = self.range(dir);
        (range.start() - 2)..=(range.end() + 2)
    }

    fn active_neighbor_count(&self, (x, y, z): Cube) -> usize {
        self.0
            .iter()
            .filter(|(nx, ny, nz)| {
                (x, y, z) != (*nx, *ny, *nz)
                    && (x - nx).abs() < 2
                    && (y - ny).abs() < 2
                    && (z - nz).abs() < 2
            })
            .count()
    }

    fn is_active(&self, cube: Cube) -> bool {
        self.0.iter().any(|c| cube == *c)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for z in self.range(Dir::Z) {
            println!("LAYER {}:", z);
            for x in self.range(Dir::X) {
                for y in self.range(Dir::Y) {
                    print!("{}", if self.is_active((x, y, z)) { '#' } else { '.' });
                }
                println!("");
            }
        }
    }
}

fn part1(s: &str) -> usize {
    let mut grid = Grid::from_str(s);
    for _ in 0..6 {
        let mut next = vec![];
        for x in grid.extended_range(Dir::X) {
            for y in grid.extended_range(Dir::Y) {
                for z in grid.extended_range(Dir::Z) {
                    let cur = (x, y, z);
                    let active_neighbors = grid.active_neighbor_count(cur);
                    if active_neighbors == 2 || active_neighbors == 3 {
                        if grid.is_active(cur) {
                            next.push(cur);
                        } else if active_neighbors == 3 {
                            next.push(cur);
                        }
                    }
                }
            }
        }
        grid = Grid::from_vec(next);
    }
    grid.0.len()
}

type HyperCube = (Int, Int, Int, Int);
struct HyperGrid(Vec<HyperCube>);

impl HyperGrid {
    fn from_str(s: &str) -> Self {
        let grid = s
            .split('\n')
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(y, c)| match c {
                        '#' => Some((x as isize, y as isize, 0 as isize, 0 as isize)),
                        _ => None,
                    })
                    .collect::<Vec<HyperCube>>()
            })
            .collect::<Vec<HyperCube>>();
        HyperGrid::from_vec(grid)
    }

    fn from_vec(vec: Vec<HyperCube>) -> Self {
        HyperGrid(vec)
    }

    fn range(&self, dir: Dir) -> RangeInclusive<Int> {
        let range = self
            .0
            .iter()
            .map(|cube| match &dir {
                Dir::X => cube.0,
                Dir::Y => cube.1,
                Dir::Z => cube.2,
                Dir::W => cube.3,
            })
            .fold((0isize, 0isize), |(cur_min, cur_max), cur| {
                (min(cur, cur_min), max(cur, cur_max))
            });
        (range.0)..=(range.1)
    }

    fn extended_range(&self, dir: Dir) -> RangeInclusive<Int> {
        let range = self.range(dir);
        (range.start() - 2)..=(range.end() + 2)
    }

    fn active_neighbor_count(&self, (x, y, z, w): HyperCube) -> usize {
        self.0
            .iter()
            .filter(|(nx, ny, nz, nw)| {
                (x, y, z, w) != (*nx, *ny, *nz, *nw)
                    && (x - nx).abs() < 2
                    && (y - ny).abs() < 2
                    && (z - nz).abs() < 2
                    && (w - nw).abs() < 2
            })
            .count()
    }

    fn is_active(&self, cube: HyperCube) -> bool {
        self.0.iter().any(|c| cube == *c)
    }
}

fn part2(s: &str) -> usize {
    let mut grid = HyperGrid::from_str(s);
    for _ in 0..6 {
        let mut next = vec![];
        for x in grid.extended_range(Dir::X) {
            for y in grid.extended_range(Dir::Y) {
                for z in grid.extended_range(Dir::Z) {
                    for w in grid.extended_range(Dir::W) {
                        let cur = (x, y, z, w);
                        let active_neighbors = grid.active_neighbor_count(cur);
                        if active_neighbors == 2 || active_neighbors == 3 {
                            if grid.is_active(cur) {
                                next.push(cur);
                            } else if active_neighbors == 3 {
                                next.push(cur);
                            }
                        }
                    }
                }
            }
        }
        grid = HyperGrid::from_vec(next);
    }
    grid.0.len()
}

fn main() {
    let input = include_str!("in.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 112);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 848);
}