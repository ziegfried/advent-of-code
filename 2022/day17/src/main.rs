// Problem: https://adventofcode.com/2022/day/17

use itertools::Itertools;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<Point> for &Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

fn point(x: i64, y: i64) -> Point {
    Point { x, y }
}

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        point(x, y)
    }
}

const SHAPE_ORIGIN: Point = Point { x: 0, y: 0 };

#[derive(Debug, Clone)]
struct Shape(Vec<Point>, Point);

impl Shape {
    fn from(points: Vec<Point>) -> Self {
        let (_, max_x) = points.iter().map(|p| p.x).minmax().into_option().unwrap();
        let (_, max_y) = points.iter().map(|p| p.y).minmax().into_option().unwrap();
        let bottom_right: Point = (max_x, max_y).into();
        Shape(points, bottom_right)
    }
    fn width(&self) -> i64 {
        self.1.x - SHAPE_ORIGIN.x + 1
    }
}
impl From<Vec<(i64, i64)>> for Shape {
    fn from(points: Vec<(i64, i64)>) -> Self {
        let points: Vec<Point> = points.iter().map(|&p| p.into()).collect();
        Shape::from(points)
    }
}

fn shapes() -> Vec<Shape> {
    vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(2, 0), (2, 1), (2, 2), (1, 0), (0, 0)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ]
    .iter()
    .cloned()
    .map(|v| v.into())
    .collect()
}

struct Rock {
    shape: Shape,
    offset: Point,
}
impl Rock {
    fn new(shape: Shape, top: i64) -> Self {
        Rock {
            shape,
            offset: point(2, top + 4),
        }
    }
    fn shift_left(&mut self) {
        if self.offset.x > 0 {
            self.offset.x -= 1;
        }
    }
    fn shift_right(&mut self) {
        let width = self.shape.width();
        if self.offset.x + width < 7 {
            self.offset.x += 1;
        }
    }
    fn shift(&mut self, dir: &Jet) {
        match dir {
            Jet::Left => self.shift_left(),
            Jet::Right => self.shift_right(),
        };
    }
    fn undo_shift(&mut self, dir: &Jet) {
        match dir {
            Jet::Left => self.shift_right(),
            Jet::Right => self.shift_left(),
        };
    }
    fn down(&mut self) {
        self.offset = self.offset + point(0, -1);
    }
    fn up(&mut self) {
        self.offset = self.offset + point(0, 1);
    }
    fn serialize_points(&self, offset: Option<Point>) -> Vec<Point> {
        let off = self.offset + offset.unwrap_or_else(|| point(0, 0));
        self.shape.0.clone().iter().map(|p| *p + off).collect()
    }
    fn collides(&self, rested: &Rested) -> bool {
        self.serialize_points(None)
            .iter()
            .any(|p| p.y < 1 || rested.get(p))
    }
}

enum Jet {
    Left,
    Right,
}

struct Rested(Vec<u8>);
impl Rested {
    fn set(&mut self, p: &Point) {
        assert!(p.y >= 0);
        let row = p.y as usize;
        while self.0.len() < row + 1 {
            self.0.push(0);
        }
        let bit = p.x as u32;
        assert!(bit <= 8);
        let mask: u8 = 2u8.pow(bit);
        self.0[row] |= mask;
    }

    fn get(&self, p: &Point) -> bool {
        assert!(p.y >= 0);
        assert!(p.x <= 8);
        let row = p.y as usize;
        let bit = p.x as u32;
        let mask: u8 = 2u8.pow(bit);
        match self.0.get(row) {
            Some(v) => v & mask == mask,
            None => false,
        }
    }
}

fn drop_shape(
    shape: &Shape,
    jets: &mut impl Iterator<Item = Jet>,
    rested: &Rested,
    top: i64,
) -> Vec<Point> {
    let rock = &mut Rock::new(shape.clone(), top);
    loop {
        let dir = &jets.next().unwrap();
        rock.shift(dir);
        if rock.collides(rested) {
            rock.undo_shift(dir);
        }
        rock.down();
        if rock.collides(rested) {
            rock.up();
            break;
        }
    }
    rock.serialize_points(None)
}

fn part1(input: &str) -> usize {
    let mut jets = input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Jet::Right,
            '<' => Jet::Left,
            _ => panic!(),
        })
        .cycle();
    let shapes = shapes();

    let mut top: i64 = 0;
    let mut rested = Rested(vec![]);

    for (i, shape) in shapes.iter().cycle().enumerate() {
        if i >= 2022 {
            break;
        }
        let points = drop_shape(shape, &mut jets, &rested, top);
        for point in points {
            top = std::cmp::max(point.y, top);
            rested.set(&point);
        }
    }

    top as usize
}

fn is_rep(values: &Vec<u8>, rep_len: usize) -> bool {
    let len = values.len();
    let l1 = len - 1;
    let l2 = len - 1 - rep_len;
    (0..rep_len).all(|o| values[l1 - o] == values[l2 - o])
}

fn part2(input: &str) -> i64 {
    let mut jets = input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Jet::Right,
            '<' => Jet::Left,
            _ => panic!(),
        })
        .cycle();
    let shapes = shapes();
    let mut shapes = shapes.iter().cycle();

    let mut top: i64 = 0;
    let mut rested = Rested(vec![]);
    let mut dropped = 0;
    let rep;

    'repfinder: loop {
        for _ in 0..10_000 {
            let points = drop_shape(shapes.next().unwrap(), &mut jets, &rested, top);
            for p in points {
                rested.set(&p);
                if p.y > top {
                    top = p.y;
                }
            }
        }
        dropped += 10_000;

        let mut len = 5;

        loop {
            if is_rep(&rested.0, len) {
                rep = len;
                break 'repfinder;
            }
            len += 5;
            if len >= rested.0.len() / 2 {
                break;
            }
        }
    }

    let idx = rested.0.len() - rep;
    let seg: Vec<u8> = Vec::from(&rested.0[idx..]);

    let mut rep_drops = 0;
    loop {
        rep_drops += 1;
        let points = drop_shape(shapes.next().unwrap(), &mut jets, &rested, top);
        for p in points {
            rested.set(&p);
            if p.y > top {
                top = p.y;
            }
        }

        if rested.0.len() - idx == 2 * rep {
            assert_eq!(Vec::from(&rested.0[(idx + rep)..]), seg);
            break;
        }
    }

    let target: i64 = 1000000000000;
    let skipped = (target - dropped - rep_drops) / rep_drops;

    let remaining = (target - dropped) % rep_drops;
    for _ in 0..remaining {
        let points = drop_shape(shapes.next().unwrap(), &mut jets, &rested, top);
        for p in points {
            rested.set(&p);
            if p.y > top {
                top = p.y;
            }
        }
    }

    top + skipped * rep as i64
}

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 3068);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 1514285714288);
}
