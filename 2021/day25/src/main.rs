fn step(grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let row_count = grid.len();
    let col_count = grid[0].len();
    let mut grid = grid.clone();
    let mut moves_east = vec![];
    for row in 0..row_count {
        for col in 0..col_count {
            if grid[row][col] == '>' {
                if grid[row][(col + 1) % col_count] == '.' {
                    moves_east.push((row, col));
                }
            }
        }
    }
    for (row, col) in moves_east.clone() {
        grid[row][col] = '.';
        grid[row][(col + 1) % col_count] = '>';
    }
    let mut moves_south = vec![];
    for row in 0..row_count {
        for col in 0..col_count {
            if grid[row][col] == 'v' {
                if grid[(row + 1) % row_count][col] == '.' {
                    moves_south.push((row, col));
                }
            }
        }
    }
    for (row, col) in moves_south.clone() {
        grid[row][col] = '.';
        grid[(row + 1) % row_count][col] = 'v';
    }

    (grid, moves_east.len() + moves_south.len())
}

fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 1.. {
        let (next, move_count) = step(&grid);
        if move_count == 0 {
            return i;
        }
        grid = next;
    }
    unreachable!()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 58);
}
