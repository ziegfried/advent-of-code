use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Number {
    Reg(usize),
    Pair(Box<Number>, Box<Number>),
}
use Number::*;

fn pair(n1: usize, n2: usize) -> Number {
    Pair(Box::new(Reg(n1)), Box::new(Reg(n2)))
}

fn map_value(value: &Value) -> Number {
    use Value::*;
    match value {
        Number(n) => Reg(n.as_u64().unwrap() as usize),
        Array(children) => {
            assert_eq!(children.len(), 2);
            let left = &children[0];
            let right = &children[1];
            Pair(Box::new(map_value(left)), Box::new(map_value(right)))
        }
        _ => panic!(),
    }
}

fn parse(input: &str) -> Number {
    let v: Value = serde_json::from_str(input).unwrap();
    map_value(&v)
}

fn split(n: Number) -> Option<Number> {
    match n {
        Reg(num) => {
            if num > 9 {
                Some(pair(num / 2, num / 2 + num % 2))
            } else {
                None
            }
        }
        Pair(n1, n2) => {
            let s1 = split(*n1.clone());
            if s1.is_some() {
                Some(Pair(
                    match s1 {
                        Some(n) => Box::new(n),
                        None => n1,
                    },
                    n2,
                ))
            } else {
                let s2 = split(*n2.clone());
                if s2.is_some() {
                    Some(Pair(
                        n1,
                        match s2 {
                            Some(n) => Box::new(n),
                            None => n2,
                        },
                    ))
                } else {
                    None
                }
            }
        }
    }
}

fn add(n1: &Number, n2: &Number) -> Number {
    Pair(Box::new(n1.clone()), Box::new(n2.clone()))
}

fn unpack_regular(n: Number) -> usize {
    match n {
        Reg(v) => v,
        _ => panic!("{:?} not a regular number", n),
    }
}

fn add_left(n: Number, a: usize) -> Number {
    match n {
        Reg(v) => Reg(v + a),
        Pair(l, r) => Pair(Box::new(add_left(*l, a)), r),
    }
}

fn add_right(n: Number, a: usize) -> Number {
    match n {
        Reg(v) => Reg(v + a),
        Pair(l, r) => Pair(l, Box::new(add_right(*r, a))),
    }
}

fn explode_inner(n: &Number, depth: usize) -> Option<(Number, usize, usize)> {
    if depth == 5 {
        if let Pair(a, b) = n.clone() {
            return Some((Reg(0), unpack_regular(*a), unpack_regular(*b)));
        }
    }
    match n {
        Pair(left, right) => {
            let left = *left.clone();
            let right = *right.clone();
            if let Some((new_left, l, r)) = explode_inner(&left, depth + 1) {
                let (right, r) = if let Reg(n) = right {
                    (Reg(r + n), 0)
                } else {
                    (add_left(right, r), 0)
                };
                return Some((Pair(Box::new(new_left), Box::new(right)), l, r));
            }
            if let Some((new_right, l, r)) = explode_inner(&right, depth + 1) {
                let (left, l) = if let Reg(n) = left {
                    (Reg(l + n), 0)
                } else {
                    (add_right(left, l), 0)
                };
                return Some((Pair(Box::new(left), Box::new(new_right.clone())), l, r));
            }
        }
        _ => {}
    }
    None
}

fn explode(n: Number) -> Option<Number> {
    if let Some((res, _, _)) = explode_inner(&n, 1) {
        return Some(res);
    } else {
        return None;
    }
}

fn reduce(n: Number) -> Number {
    match explode(n.clone()) {
        Some(exploded) => reduce(exploded),
        None => match split(n.clone()) {
            Some(split) => reduce(split),
            None => n,
        },
    }
}

fn magnitude(v: Number) -> usize {
    match v {
        Pair(a, b) => 3 * magnitude(*a) + 2 * magnitude(*b),
        Reg(v) => v,
    }
}

#[allow(unused)]
fn number_to_str(n: &Number) -> String {
    match n {
        Reg(v) => format!("{}", v),
        Pair(a, b) => format!("[{},{}]", number_to_str(a), number_to_str(b)),
    }
}

fn part1(input: &str) -> usize {
    let result = input
        .lines()
        .map(parse)
        .reduce(|a, b| reduce(add(&a, &b)))
        .unwrap();
    magnitude(result)
}

fn part2(input: &str) -> usize {
    let inputs: Vec<Number> = input.lines().map(parse).collect();
    let mut max_val: usize = 0;
    for a in inputs.iter() {
        for b in inputs.iter() {
            if a != b {
                max_val = usize::max(max_val, magnitude(reduce(add(a, b))));
                max_val = usize::max(max_val, magnitude(reduce(add(b, a))));
            }
        }
    }
    max_val
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
        split(parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")),
        Some(parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"))
    );
    assert_eq!(split(parse("[1,2]")), None);

    assert_eq!(
        split(parse(
            "[[[[4, 0], [5, 4]], [[7, 0], [15, 5]]], [10, [[11, 9], [11, 0]]]]"
        )),
        Some(parse(
            "[[[[4, 0], [5, 4]], [[7, 0], [[7, 8], 5]]], [10, [[11, 9], [11, 0]]]]"
        ))
    );
}

#[test]
fn test_magnitude() {
    assert_eq!(magnitude(parse("[[1,2],[[3,4],5]]")), 143);
    assert_eq!(
        magnitude(parse(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        )),
        3488
    );
}

#[test]
fn test_explode() {
    assert_eq!(
        explode(parse("[[[[[9,8],1],2],3],4]")),
        Some(parse("[[[[0,9],2],3],4]"))
    );
    assert_eq!(
        explode(parse("[7,[6,[5,[4,[3,2]]]]]")),
        Some(parse("[7,[6,[5,[7,0]]]]"))
    );
    assert_eq!(
        explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
        Some(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
    );
    assert_eq!(
        explode(parse("[7,[6,[5,[4,[3,2]]]]]")),
        Some(parse("[7,[6,[5,[7,0]]]]"))
    );
    assert_eq!(
        explode(parse("[[6,[5,[4,[3,2]]]],1]")),
        Some(parse("[[6,[5,[7,0]]],3]"))
    );
    assert_eq!(
        explode(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")),
        Some(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
    );
    assert_eq!(
        explode(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
        Some(parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"))
    );
    assert_eq!(
        explode(parse(
            "[[[[4,0],[5,4]],[[7,7],[6,5]]],[[[5,5],[0,6]],[[6,5],[5,5]]]]"
        )),
        None
    );
}
