fn trees_on_slope(lines: &Vec<&str>, (right, down): (usize, usize)) -> usize {
    let mut i = 0;
    let mut trees = 0;
    let mut row = 0;
    for line in lines.clone().iter() {
        if down == 1 || row % 2 == 0 {
            let cur = line.chars().nth(i).unwrap();
            i = (i + right) % line.len();
            if cur == '#' {
                trees += 1;
            }
        }
        row += 1;
    }
    return trees;
}

fn main() {
    let lines = include_str!("../in.txt").split("\n").collect::<Vec<&str>>();
    println!("Part 1: {}", trees_on_slope(&lines, (3, 1)));
    let res = vec![
        trees_on_slope(&lines, (1, 1)),
        trees_on_slope(&lines, (3, 1)),
        trees_on_slope(&lines, (5, 1)),
        trees_on_slope(&lines, (7, 1)),
        trees_on_slope(&lines, (1, 2)),
    ];
    println!("Part 2: {}", res.iter().fold(1, |a, b| a * b));
}
