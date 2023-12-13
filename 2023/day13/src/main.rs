// Problem: https://adventofcode.com/2023/day/13

type Result = usize;

type Grid = Vec<Vec<bool>>;

type Input = Vec<Grid>;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n\n")
        .map(|input| {
            input
                .lines()
                .map(|line| line.chars().map(|ch| ch == '#').collect::<Vec<bool>>())
                .collect::<Grid>()
        })
        .collect::<Vec<Grid>>()
}

fn rotate(grid: &Grid) -> Grid {
    let mut new_grid = vec![];
    for c in 0..grid[0].len() {
        let r = (0..grid.len()).map(|r| grid[r][c]).collect::<Vec<bool>>();
        new_grid.push(r);
    }
    new_grid
}

fn is_reflection(grid: &Grid, split: usize) -> bool {
    let to_edge = std::cmp::min(grid.len() - split, split);
    (0..to_edge).all(|offset| grid[split + offset] == grid[split - offset - 1])
}

fn find_reflection(grid: &Grid) -> Option<usize> {
    (1..(grid.len())).find(|&split| is_reflection(grid, split))
}

fn reflection_score(grid: &Grid) -> Option<usize> {
    find_reflection(grid)
        .map(|idx| idx * 100)
        .or_else(|| find_reflection(&rotate(grid)))
}

fn part1(input: &Input) -> Result {
    input.iter().map(reflection_score).map(|v| v.unwrap()).sum()
}

#[test]
fn test_part1() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part1(&input), 405);
}

fn find_all_reflections(grid: &Grid) -> Vec<usize> {
    (1..grid.len())
        .filter(|split| is_reflection(grid, *split))
        .collect()
}

fn alt_score(grid: &Grid) -> usize {
    let orig = reflection_score(grid).unwrap();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let mut grid = grid.clone();
            grid[r][c] = !grid[r][c];
            for split in find_all_reflections(&grid) {
                if split * 100 != orig {
                    return split * 100;
                }
            }
            for split in find_all_reflections(&rotate(&grid)) {
                if split != orig {
                    return split;
                }
            }
        }
    }
    unreachable!()
}

fn part2(input: &Input) -> Result {
    input.iter().map(alt_score).sum()
}

#[test]
fn test_part2() {
    let input = parse_input(include_str!("test.txt"));
    assert_eq!(part2(&input), 400);
}

// ------------------------------------------

fn main() {
    let input = parse_input(include_str!("input.txt"));
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}
