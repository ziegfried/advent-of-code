use pathfinding::prelude::dijkstra;

fn smallest_risk(grid: &Vec<Vec<usize>>) -> usize {
    let row_count = grid.len();
    let col_count = grid[0].len();
    let dirs: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let result = dijkstra(
        &(0, 0),
        |&(x, y)| {
            dirs.iter()
                .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|&(x, y)| x >= 0 && y >= 0)
                .map(|(x, y)| (x as usize, y as usize))
                .filter(|&(x, y)| x < row_count && y < col_count)
                .map(|(x, y)| ((x, y), grid[x][y]))
        },
        |&(row, col)| row == row_count - 1 && col == col_count - 1,
    );
    let (_, result) = result.unwrap();
    result
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    smallest_risk(&grid)
}

fn part2(input: &str) -> usize {
    let grid0: Vec<Vec<usize>> = input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let times = 5;
    let row_count = grid0.len();
    let col_count = grid0[0].len();
    let grid = (0..(row_count * times))
        .map(|row| {
            (0..(col_count * times))
                .map(|col| {
                    let val = grid0[row % row_count][col % col_count]
                        + (row / row_count)
                        + (col / col_count);
                    if val > 9 {
                        val - 9
                    } else {
                        val
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    smallest_risk(&grid)
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 40);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 315);
}
