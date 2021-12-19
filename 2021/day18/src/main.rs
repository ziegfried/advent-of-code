use itertools::Itertools;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq, Eq)]
enum Number {
    Reg(usize),
    Pair(Box<Number>, Box<Number>),
}
impl Debug for Number {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fn number_to_string(n: &Number) -> String {
            match n {
                Reg(v) => format!("{}", v),
                Pair(a, b) => format!("[{},{}]", number_to_string(a), number_to_string(b)),
            }
        }
        formatter.write_str(&number_to_string(self))
    }
}
use Number::*;

fn pair(n1: Number, n2: Number) -> Number {
    Pair(Box::new(n1), Box::new(n2))
}

fn parse(input: &str) -> Number {
    fn map_json_value(value: &serde_json::Value) -> Number {
        use serde_json::Value::*;
        match value {
            Number(n) => Reg(n.as_u64().unwrap() as usize),
            Array(children) => {
                assert_eq!(children.len(), 2);
                pair(map_json_value(&children[0]), map_json_value(&children[1]))
            }
            _ => panic!("invalid number string: unexpected value type {:?}", value),
        }
    }
    map_json_value(&serde_json::from_str(input).expect("invalid number string"))
}

fn split(n: &Number) -> Option<Number> {
    match n {
        Reg(num) => {
            if *num > 9 {
                Some(pair(Reg(num / 2), Reg(num / 2 + num % 2)))
            } else {
                None
            }
        }
        Pair(n1, n2) => {
            if let Some(s1) = split(&n1) {
                Some(Pair(Box::new(s1), n2.clone()))
            } else if let Some(s2) = split(&n2) {
                Some(Pair(n1.clone(), Box::new(s2)))
            } else {
                None
            }
        }
    }
}

fn explode(n: &Number) -> Option<Number> {
    fn add_to_side(n: &Number, val: usize, left_side: bool) -> Number {
        match n {
            Reg(v) => Reg(v + val),
            Pair(l, r) => match left_side {
                true => Pair(Box::new(add_to_side(l, val, true)), r.clone()),
                false => Pair(l.clone(), Box::new(add_to_side(r, val, false))),
            },
        }
    }
    fn unwrap_regular(n: &Number) -> usize {
        match n {
            Reg(v) => *v,
            _ => panic!("{:?} not a regular number", n),
        }
    }
    fn explode_layer(n: &Number, depth: usize) -> Option<(Number, usize, usize)> {
        if let Pair(left, right) = n {
            if depth == 4 {
                return Some((Reg(0), unwrap_regular(left), unwrap_regular(right)));
            }
            if let Some((left, left_over, right_over)) = explode_layer(left, depth + 1) {
                let right = if let Reg(n) = **right {
                    Reg(n + right_over)
                } else {
                    add_to_side(right, right_over, true)
                };
                return Some((Pair(Box::new(left), Box::new(right)), left_over, 0));
            }
            if let Some((right, left_over, right_over)) = explode_layer(right, depth + 1) {
                let left = if let Reg(n) = **left {
                    Reg(n + left_over)
                } else {
                    add_to_side(left, left_over, false)
                };
                return Some((Pair(Box::new(left), Box::new(right)), 0, right_over));
            }
        }
        None
    }

    explode_layer(n, 0).map(|(res, _, _)| res)
}

fn reduce(n: &Number) -> Number {
    match explode(n) {
        Some(exploded) => reduce(&exploded),
        None => match split(n) {
            Some(split) => reduce(&split),
            None => n.clone(),
        },
    }
}

fn add(n1: &Number, n2: &Number) -> Number {
    reduce(&pair(n1.clone(), n2.clone()))
}

fn magnitude(v: &Number) -> usize {
    match v {
        Pair(a, b) => 3 * magnitude(a) + 2 * magnitude(b),
        Reg(v) => *v,
    }
}

fn part1(input: &str) -> usize {
    let result = input.lines().map(parse).reduce(|a, b| add(&a, &b)).unwrap();
    magnitude(&result)
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .permutations(2)
        .map(|perm| magnitude(&add(&perm[0], &perm[1])))
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test1.txt")), 3488);
    assert_eq!(part1(include_str!("test2.txt")), 4140);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test2.txt")), 3993);
}

#[test]
fn test_split() {
    assert_eq!(
        split(&parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")),
        Some(parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"))
    );
    assert_eq!(split(&parse("[1,2]")), None);
    assert_eq!(
        split(&parse(
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,9],[11,0]]]]"
        )),
        Some(parse(
            "[[[[4,0],[5,4]],[[7,0],[[7,8],5]]],[10,[[11,9],[11,0]]]]"
        ))
    );
}

#[test]
fn test_magnitude() {
    assert_eq!(magnitude(&parse("[[1,2],[[3,4],5]]")), 143);
    assert_eq!(
        magnitude(&parse(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        )),
        3488
    );
}

#[test]
fn test_explode() {
    assert_eq!(
        explode(&parse("[[[[[9,8],1],2],3],4]")),
        Some(parse("[[[[0,9],2],3],4]"))
    );
    assert_eq!(
        explode(&parse("[7,[6,[5,[4,[3,2]]]]]")),
        Some(parse("[7,[6,[5,[7,0]]]]"))
    );
    assert_eq!(
        explode(&parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
        Some(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
    );
    assert_eq!(
        explode(&parse("[7,[6,[5,[4,[3,2]]]]]")),
        Some(parse("[7,[6,[5,[7,0]]]]"))
    );
    assert_eq!(
        explode(&parse("[[6,[5,[4,[3,2]]]],1]")),
        Some(parse("[[6,[5,[7,0]]],3]"))
    );
    assert_eq!(
        explode(&parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
        Some(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
    );
    assert_eq!(
        explode(&parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
        Some(parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
    );
    assert_eq!(
        explode(&parse(
            "[[[[4,0],[5,4]],[[7,7],[6,5]]],[[[5,5],[0,6]],[[6,5],[5,5]]]]"
        )),
        None
    );
}
