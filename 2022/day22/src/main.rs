#![allow(
    dead_code,
    unused_variables,
    unreachable_code,
    clippy::diverging_sub_expression
)]

use std::collections::HashMap;

// Problem: https://adventofcode.com/2022/day/22
use itertools::Itertools;
use Direction::*;

type Point = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
    fn turn_around(&self) -> Self {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
    fn go(&self, &(row, col): &Point) -> Point {
        match self {
            Up => (row - 1, col),
            Right => (row, col + 1),
            Down => (row + 1, col),
            Left => (row, col - 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Open,
    Wall,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            ' ' => Space::Empty,
            '.' => Space::Open,
            '#' => Space::Wall,
            _ => panic!("unkown char {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    pos: Point,
    facing: Direction,
}

fn parse_board(input: &str) -> Board {
    let mut map: HashMap<Point, Space> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            map.insert((row as i32, col as i32), Space::from_char(ch));
        }
    }

    Board(map)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Turn {
    Right,
    Left,
}

#[derive(Debug, Clone)]
enum Instruction {
    Go(usize),
    Turn(Turn),
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut result = vec![];
    let mut cur = vec![];
    for ch in input.chars() {
        match ch {
            'R' => {
                if !cur.is_empty() {
                    let n = cur.iter().collect::<String>().parse::<usize>().unwrap();
                    result.push(Instruction::Go(n));
                    cur = vec![];
                }
                result.push(Instruction::Turn(Turn::Right));
            }
            'L' => {
                if !cur.is_empty() {
                    let n = cur.iter().collect::<String>().parse::<usize>().unwrap();
                    result.push(Instruction::Go(n));
                    cur = vec![];
                }
                result.push(Instruction::Turn(Turn::Left));
            }
            _ => {
                cur.push(ch);
            }
        };
    }
    if !cur.is_empty() {
        let n = cur.iter().collect::<String>().parse::<usize>().unwrap();
        result.push(Instruction::Go(n));
    }

    result
}

#[derive(Debug, Clone)]
struct Board(HashMap<Point, Space>);
impl Board {
    fn get(&self, p: Point) -> Space {
        match self.0.get(&p) {
            Some(v) => *v,
            None => Space::Empty,
        }
    }
    fn row_bounds(&self, row: i32) -> (i32, i32) {
        self.0
            .iter()
            .filter(|(&(r, _), &v)| row == r && v != Space::Empty)
            .map(|((_, col), _)| col)
            .cloned()
            .minmax()
            .into_option()
            .unwrap()
    }
    fn col_bounds(&self, col: i32) -> (i32, i32) {
        self.0
            .iter()
            .filter(|(&(_, c), &v)| col == c && v != Space::Empty)
            .map(|((row, _), _)| row)
            .cloned()
            .minmax()
            .into_option()
            .unwrap()
    }

    fn bounds(&self) -> (Point, Point) {
        let (&min_row, &max_row) = self
            .0
            .keys()
            .map(|(row, _)| row)
            .minmax()
            .into_option()
            .unwrap();
        let (&min_col, &max_col) = self
            .0
            .keys()
            .map(|(_, col)| col)
            .minmax()
            .into_option()
            .unwrap();

        ((min_row, min_col), (max_row, max_col))
    }

    fn print(&self, state: &State) {
        let ((min_row, min_col), (max_row, max_col)) = self.bounds();

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let p = (row, col);
                if p == state.pos {
                    match state.facing {
                        Up => print!("^"),
                        Right => print!(">"),
                        Down => print!("v"),
                        Left => print!("<"),
                    }
                } else {
                    match self.get(p) {
                        Space::Empty => print!(" "),
                        Space::Open => print!("."),
                        Space::Wall => print!("#"),
                    }
                }
            }
            println!();
        }
    }
    fn print_with_moves(&self, moves: &HashMap<Point, Direction>) {
        let ((min_row, min_col), (max_row, max_col)) = self.bounds();

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let p = (row, col);
                if let Some(dir) = moves.get(&p) {
                    match dir {
                        Up => print!("^"),
                        Right => print!(">"),
                        Down => print!("v"),
                        Left => print!("<"),
                    }
                } else {
                    match self.get(p) {
                        Space::Empty => print!(" "),
                        Space::Open => print!("."),
                        Space::Wall => print!("#"),
                    }
                }
            }
            println!();
        }
    }
}

fn part1(input: &str) -> i32 {
    let (board, instructions) = input.split_once("\n\n").unwrap();
    let board = parse_board(board);
    let instructions = parse_instructions(instructions.trim());

    let mut state = State {
        pos: (0, board.row_bounds(0).0),
        facing: Direction::Right,
    };

    let mut moves = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Go(distance) => {
                for _ in 0..distance {
                    moves.insert(state.pos, state.facing);
                    let pos = state.pos;
                    let facing = state.facing;
                    let next_pos = facing.go(&pos);
                    match board.get(next_pos) {
                        Space::Open => {
                            state = State {
                                pos: next_pos,
                                facing,
                            };
                        }
                        Space::Wall => {
                            break;
                        }
                        Space::Empty => {
                            let next = match state.facing {
                                Up => (board.col_bounds(state.pos.1).1, state.pos.1),
                                Right => (state.pos.0, board.row_bounds(state.pos.0).0),
                                Down => (board.col_bounds(state.pos.1).0, state.pos.1),
                                Left => (state.pos.0, board.row_bounds(state.pos.0).1),
                            };
                            match board.get(next) {
                                Space::Empty => panic!("wrap around is empty"),
                                Space::Open => {
                                    state = State { pos: next, facing };
                                }
                                Space::Wall => {
                                    break;
                                }
                            }
                        }
                    }
                    moves.insert(state.pos, state.facing);
                }
            }
            Instruction::Turn(turn) => match turn {
                Turn::Right => {
                    state = State {
                        pos: state.pos,
                        facing: state.facing.turn_right(),
                    }
                }
                Turn::Left => {
                    state = State {
                        pos: state.pos,
                        facing: state.facing.turn_left(),
                    }
                }
            },
        }
        moves.insert(state.pos, state.facing);
    }

    // board.print_with_moves(&moves);

    let (row, col) = state.pos;
    let face_value: i32 = match state.facing {
        Up => 3,
        Right => 0,
        Down => 1,
        Left => 2,
    };

    (row + 1) * 1000 + (col + 1) * 4 + face_value
}

fn part2(input: &str, square_size: i32) -> i32 {
    let (board, instructions) = input.split_once("\n\n").unwrap();
    let board = parse_board(board);
    let instructions = parse_instructions(instructions.trim());

    let mut state = State {
        pos: (0, board.row_bounds(0).0),
        facing: Direction::Right,
    };

    let mut moves = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Go(distance) => {
                for _ in 0..distance {
                    moves.insert(state.pos, state.facing);
                    let pos = state.pos;
                    let facing = state.facing;
                    let next_pos = facing.go(&pos);
                    match board.get(next_pos) {
                        Space::Open => {
                            state = State {
                                pos: next_pos,
                                facing,
                            };
                        }
                        Space::Wall => {
                            break;
                        }
                        Space::Empty => {
                            let (next, next_dir) =
                                wrap_on_cube(state.pos, state.facing, &board, square_size);
                            match board.get(next) {
                                Space::Empty => panic!("wrap around is empty"),
                                Space::Open => {
                                    state = State {
                                        pos: next,
                                        facing: next_dir,
                                    };
                                }
                                Space::Wall => {
                                    break;
                                }
                            }
                        }
                    }
                    moves.insert(state.pos, state.facing);
                }
            }
            Instruction::Turn(turn) => match turn {
                Turn::Right => {
                    state = State {
                        pos: state.pos,
                        facing: state.facing.turn_right(),
                    }
                }
                Turn::Left => {
                    state = State {
                        pos: state.pos,
                        facing: state.facing.turn_left(),
                    }
                }
            },
        }
        moves.insert(state.pos, state.facing);
    }

    // board.print_with_moves(&moves);

    let (row, col) = state.pos;
    let face_value: i32 = match state.facing {
        Up => 3,
        Right => 0,
        Down => 1,
        Left => 2,
    };

    (row + 1) * 1000 + (col + 1) * 4 + face_value
}

fn wrap_on_cube((row, col): Point, dir: Direction, board: &Board, sqs: i32) -> (Point, Direction) {
    let seg = (row / sqs, col / sqs);

    let translate = |(row, col): Point,
                     src_seg: Point,
                     src_dir: Direction,
                     dst_seg: Point,
                     dst_dir: Direction,
                     flip: bool|
     -> (Point, Direction) {
        let mut n = match src_dir {
            Up => col,
            Right => row,
            Down => col,
            Left => row,
        } % sqs;
        if flip {
            n = sqs - n - 1;
        }

        let dst_pos = match dst_dir {
            Up => (dst_seg.0 * sqs, dst_seg.1 * sqs + n),
            Right => (dst_seg.0 * sqs + n, (dst_seg.1 + 1) * sqs - 1),
            Down => ((dst_seg.0 + 1) * sqs - 1, dst_seg.1 * sqs + n),
            Left => (dst_seg.0 * sqs + n, dst_seg.1 * sqs),
        };

        (dst_pos, dst_dir.turn_around())
    };

    if sqs == 50 {
        match (seg, dir) {
            ((0, 1), Up) => translate((row, col), seg, dir, (3, 0), Left, false),
            ((0, 1), Left) => translate((row, col), seg, dir, (2, 0), Left, true),
            ((0, 2), Up) => translate((row, col), seg, dir, (3, 0), Down, false),
            ((0, 2), Right) => translate((row, col), seg, dir, (2, 1), Right, true),
            ((0, 2), Down) => translate((row, col), seg, dir, (1, 1), Right, false),
            ((1, 1), Left) => translate((row, col), seg, dir, (2, 0), Up, false),
            ((1, 1), Right) => translate((row, col), seg, dir, (0, 2), Down, false),
            ((2, 0), Up) => translate((row, col), seg, dir, (1, 1), Left, false),
            ((2, 0), Left) => translate((row, col), seg, dir, (0, 1), Left, true),
            ((2, 1), Right) => translate((row, col), seg, dir, (0, 2), Right, true),
            ((2, 1), Down) => translate((row, col), seg, dir, (3, 0), Right, false),
            ((3, 0), Left) => translate((row, col), seg, dir, (0, 1), Up, false),
            ((3, 0), Right) => translate((row, col), seg, dir, (2, 1), Down, false),
            ((3, 0), Down) => translate((row, col), seg, dir, (0, 2), Up, false),
            _ => panic!("invalid wrap {:?} {:?}", seg, dir),
        }
    // } else {
    //     match (seg, dir) {
    //         ((0, 3), Left) => translate((row, col), seg, dir, (3, 0), Left, false),
    //         _ => panic!("invalid wrap {:?} {:?}", seg, dir),
    //     }

    } else {
        panic!("unknown cube size")
    }
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt"), 50));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 6032);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt"), 4), 5031);
}
