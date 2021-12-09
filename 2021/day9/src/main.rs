use std::collections::HashSet;

type Grid = Vec<Vec<usize>>;

fn parse_grid(input: &str) -> Grid {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn neighbors(x: usize, y: usize, grid: &Grid) -> Vec<(usize, usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    vec![(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .filter_map(|(dx, dy)| {
            let x1 = (x as isize) + dx;
            let y1 = (y as isize) + dy;
            if x1 >= 0 && y1 >= 0 {
                let x1 = x1 as usize;
                let y1 = y1 as usize;
                if x1 < rows && y1 < cols {
                    Some((x1 as usize, y1 as usize, grid[x1 as usize][y1 as usize]))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn low_points(grid: &Grid) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = vec![];
    for x in 0..rows {
        for y in 0..cols {
            let v = grid[x][y];
            if neighbors(x, y, grid).iter().all(|(_, _, nv)| v < *nv) {
                result.push((x, y));
            }
        }
    }
    result
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    low_points(&grid)
        .iter()
        .map(|(x, y)| grid[*x][*y] + 1)
        .sum()
}

fn crawl_basin(x: usize, y: usize, basin: &mut HashSet<(usize, usize)>, grid: &Grid) {
    let point = (x, y);
    if basin.contains(&point) {
        return;
    }
    basin.insert((x, y));
    let v = grid[x][y];
    for (nx, ny, nv) in neighbors(x, y, grid) {
        if nv < 9 && nv > v {
            crawl_basin(nx, ny, basin, grid);
        }
    }
}

fn part2(input: &str) -> usize {
    let grid = parse_grid(input);
    let lp = low_points(&grid);
    let basins = lp
        .iter()
        .map(|(x, y)| {
            let mut basin = HashSet::<(usize, usize)>::new();
            crawl_basin(*x, *y, &mut basin, &grid);
            basin
        })
        .collect::<Vec<_>>();
    let mut basin_sizes = basins.iter().map(|b| b.len()).collect::<Vec<usize>>();
    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).fold(1, |a, b| a * b)
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 15);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 1134);
}
