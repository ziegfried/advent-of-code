#[derive(PartialEq)]
enum BoardItem {
    Called,
    Uncalled(usize),
}
impl Default for BoardItem {
    fn default() -> Self {
        BoardItem::Called
    }
}
use BoardItem::*;

struct Board([[BoardItem; 5]; 5]);

impl Board {
    fn from_str(input: &str) -> Self {
        let mut board: [[BoardItem; 5]; 5] = Default::default();
        let lines = input.split('\n');

        for (x, line) in lines.enumerate() {
            let numbers = line
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap());
            for (y, number) in numbers.enumerate() {
                board[x][y] = Uncalled(number);
            }
        }

        Board(board)
    }
    fn bingo(&self) -> bool {
        for x in 0..5 {
            if (0..5).all(|y| self.0[x][y] == Called) {
                return true;
            }
        }
        for y in 0..5 {
            if (0..5).all(|x| self.0[x][y] == Called) {
                return true;
            }
        }
        false
    }
    fn mark_number(&mut self, number: usize) {
        for x in 0..5 {
            for y in 0..5 {
                if self.0[x][y] == Uncalled(number) {
                    self.0[x][y] = Called;
                }
            }
        }
    }
    fn score(&self) -> usize {
        (0..5)
            .map(|x| {
                self.0[x]
                    .iter()
                    .map(|item| match item {
                        Called => 0,
                        Uncalled(v) => *v,
                    })
                    .sum::<usize>()
            })
            .sum()
    }
    #[allow(dead_code)]
    fn print(&self) {
        let rows = &self.0;
        for row in rows {
            let line = row
                .iter()
                .map(|v| {
                    format!(
                        "{}",
                        match v {
                            Called => "XX".to_string(),
                            Uncalled(v) => format!("{:2}", v),
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", line);
        }
    }
}

fn part1(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let drawn_numbers = parts[0]
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = parts[1..]
        .iter()
        .map(|b| Board::from_str(b))
        .collect::<Vec<Board>>();

    for drawn_number in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_number(drawn_number);
        }

        for i in 0..boards.len() {
            let board = boards.get(i).unwrap();
            if board.bingo() {
                return board.score() * drawn_number;
            }
        }
    }

    panic!("no bingo");
}

fn part2(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let drawn_numbers = parts[0]
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = parts[1..]
        .iter()
        .map(|b| Board::from_str(b))
        .collect::<Vec<Board>>();

    for drawn_number in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark_number(drawn_number);
        }

        if boards.len() == 1 {
            if boards[0].bingo() {
                return boards[0].score() * drawn_number;
            }
        } else {
            boards.retain(|b| !b.bingo());
        }
    }

    panic!("no bingo");
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 4512);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 1924);
}
