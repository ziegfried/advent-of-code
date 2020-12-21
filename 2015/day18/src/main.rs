use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    size: usize,
    on: HashSet<(usize, usize)>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let size = lines.len();
        let mut on = HashSet::new();
        for (x, line) in lines.iter().enumerate() {
            for (y, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        on.insert((x, y));
                    }
                    '.' => {
                        // off
                    }
                    _ => {
                        return Err(());
                    }
                }
            }
        }

        Ok(Grid { on: on, size: size })
    }
}

impl Grid {
    #[allow(dead_code)]
    fn print(&self) {
        for x in 0..self.size {
            println!(
                "{}",
                (0..self.size)
                    .map(|y| if self.on.contains(&(x, y)) { '#' } else { '.' })
                    .collect::<String>()
            );
        }
    }

    fn light_corners(&mut self) {
        self.on.insert((0, 0));
        self.on.insert((0, self.size - 1));
        self.on.insert((self.size - 1, 0));
        self.on.insert((self.size - 1, self.size - 1));
    }

    fn next(&self) -> Grid {
        let size = self.size;
        let mut next = vec![];

        for x in 0..size {
            for y in 0..size {
                let neighbors = (-1..=1)
                    .flat_map(|dx| (-1..=1).map(move |dy| (x as isize + dx, y as isize + dy)))
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < size as isize && *y < size as isize)
                    .map(|(x, y)| (x as usize, y as usize))
                    .filter(|(nx, ny)| (nx, ny) != (&x, &y))
                    .filter(|(x, y)| self.on.contains(&(*x, *y)))
                    .count();

                let on = if self.on.contains(&(x, y)) {
                    neighbors == 2 || neighbors == 3
                } else {
                    neighbors == 3
                };

                if on {
                    next.push((x, y));
                }
            }
        }
        Grid {
            on: next.iter().cloned().collect::<HashSet<(usize, usize)>>(),
            size: size,
        }
    }

    fn on_count(&self) -> usize {
        self.on.len()
    }
}

fn part1(s: &str, steps: usize) -> usize {
    let mut grid: Grid = s.parse().unwrap();
    for _ in 0..steps {
        grid = grid.next();
    }
    grid.on_count()
}

fn part2(s: &str, steps: usize) -> usize {
    let mut grid: Grid = s.parse().unwrap();
    grid.light_corners();
    for _ in 0..steps {
        grid = grid.next();
        grid.light_corners();
    }
    grid.on_count()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt"), 100));
    println!("Part 2: {}", part2(include_str!("in.txt"), 100));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt"), 4), 4);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt"), 5), 17);
}
