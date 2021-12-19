use std::str::Chars;

fn part1(input: &str, cols: usize, rows: usize) -> usize {
    let len: usize = input.len();
    let mut row: usize = 0;
    let mut min_zeros: usize = usize::MAX;
    let mut result = 0;
    while row * cols < len {
        let chars = &input[(row * cols)..(row * cols + rows * cols)].chars();
        let zeros = chars.clone().filter(|c| *c == '0').count();
        if zeros < min_zeros {
            result = chars.clone().filter(|c| *c == '1').count()
                * chars.clone().filter(|c| *c == '2').count();
            min_zeros = zeros;
        }
        row += rows;
    }
    result
}

fn pixel(layers: &Vec<Chars>, i: usize) -> char {
    for layer in layers {
        match layer.clone().nth(i) {
            Some('0') => {
                return '0';
            }
            Some('1') => {
                return '1';
            }
            _ => {}
        }
    }
    '2'
}

fn part2(input: &str, cols: usize, rows: usize) -> String {
    let len: usize = input.len();
    let mut row: usize = 0;
    let mut layers: Vec<Chars> = vec![];
    while row * cols < len {
        let chars = &input[(row * cols)..(row * cols + rows * cols)].chars();
        layers.push(chars.clone());
        row += rows;
    }
    let final_image = (0..(rows * cols))
        .map(|i| pixel(&layers, i))
        .collect::<Vec<_>>();
    let enc = (0..rows)
        .map(|r| {
            (0..cols)
                .map(|c| {
                    match final_image.get(r * cols + c) {
                        Some('0') => " ",
                        Some('1') => "#",
                        Some('2') => ".",
                        _ => panic!(),
                    }
                    .to_string()
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n");
    enc
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt"), 25, 6));
    println!("Part 2:\n{}\n", part2(include_str!("in.txt"), 25, 6));
}

#[test]
fn test_part2() {
    assert_eq!(part2("0222112222120000", 2, 2), " #\n# ".to_string());
}
