const GRID_SIZE: usize = 10;
type Grid = [[u16; GRID_SIZE]; GRID_SIZE];

fn cascade_flash(grid: &mut Grid, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut flashes = vec![(x, y)];
    let dirs = vec![-1, 0, 1];
    for dx in dirs.iter() {
        for dy in dirs.iter() {
            let x1 = x as isize + dx;
            let y1 = y as isize + dy;
            if x1 >= 0 && y1 >= 0 {
                let x1 = x1 as usize;
                let y1 = y1 as usize;
                if x1 < GRID_SIZE && y1 < GRID_SIZE && !(x == x1 && y == y1) {
                    let v = grid[x1][y1];
                    grid[x1][y1] += 1;
                    if v == 9 {
                        for f in cascade_flash(grid, x1, y1) {
                            flashes.push(f);
                        }
                    }
                }
            }
        }
    }
    flashes
}

fn step(grid: &mut Grid) -> usize {
    let coords = (0..GRID_SIZE)
        .flat_map(|x| (0..GRID_SIZE).map(move |y| (x, y)))
        .collect::<Vec<(usize, usize)>>();
    for (x, y) in coords.iter() {
        grid[*x][*y] += 1;
    }
    let initial_flashes = coords
        .iter()
        .filter(|(x, y)| grid[*x][*y] > 9)
        .collect::<Vec<_>>();
    let cascaded_flashes = initial_flashes
        .iter()
        .flat_map(|(x, y)| cascade_flash(grid, *x, *y))
        .collect::<Vec<(usize, usize)>>();
    for (x, y) in cascaded_flashes.iter() {
        grid[*x][*y] = 0;
    }
    cascaded_flashes.len()
}

fn parse_grid(grid: &mut Grid, input: &str) {
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            grid[x][y] = String::from(ch).parse::<u16>().unwrap();
        }
    }
}

fn part1(input: &str) -> usize {
    let mut grid: Grid = [[0u16; GRID_SIZE]; GRID_SIZE];
    parse_grid(&mut grid, input);
    let mut result = 0;
    for _ in 0..100 {
        result += step(&mut grid);
    }
    result
}

fn part2(input: &str) -> usize {
    let mut grid: Grid = [[0u16; GRID_SIZE]; GRID_SIZE];
    parse_grid(&mut grid, input);
    let mut result = 1;
    loop {
        if step(&mut grid) != 100 {
            result += 1;
        } else {
            break;
        }
    }
    result
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 1656);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 195);
}
