fn smallest_risk(grid: &Vec<Vec<usize>>) -> usize {
    let row_count = grid.len();
    let col_count = grid[0].len();
    let mut q = vec![(0, 0, 0)];
    let mut smallest_yet = (0..row_count)
        .map(|_| (0..col_count).map(|_| None).collect::<Vec<Option<usize>>>())
        .collect::<Vec<_>>();
    while !q.is_empty() {
        let mut next_q = vec![];
        for (x, y, risk) in q {
            let next_risk = risk + grid[x][y];
            let prev = smallest_yet[x][y];
            if prev.is_none() || next_risk < prev.unwrap() {
                smallest_yet[x][y] = Some(next_risk);
                for (dx, dy) in vec![(-1, 0), (1, 0), (0, 1), (0, -1)].into_iter() {
                    let x1 = x as isize + dx;
                    let y1 = y as isize + dy;
                    if x1 >= 0 && y1 >= 0 {
                        let x1 = x1 as usize;
                        let y1 = y1 as usize;
                        if x1 < row_count && y1 < col_count {
                            next_q.push((x1, y1, next_risk));
                        }
                    }
                }
            }
        }
        q = next_q;
    }
    smallest_yet[row_count - 1][col_count - 1].unwrap() - grid[0][0]
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
