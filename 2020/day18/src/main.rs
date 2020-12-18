use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Num(usize),
    Plus,
    Times,
    Expr(Vec<Token>),
}

fn parse(it: &mut Peekable<Chars>) -> Vec<Token> {
    let mut tokens = vec![];
    loop {
        let next = it.next();
        match next {
            Some(c) => match c {
                '(' => {
                    let mut sub = vec![c];
                    let mut level = 1;
                    while let Some(c) = it.next() {
                        sub.push(c);
                        match c {
                            '(' => {
                                level += 1;
                            }
                            ')' => {
                                level -= 1;
                                if level == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    let sub_expr = sub[1..(sub.len() - 1)].iter().collect::<String>();
                    let mut sub_it = sub_expr.chars().peekable();
                    tokens.push(Token::Expr(parse(&mut sub_it)));
                }
                '+' => {
                    tokens.push(Token::Plus);
                }
                '*' => {
                    tokens.push(Token::Times);
                }
                c if c.is_digit(10) => {
                    let mut sub = vec![c];
                    while let Some(c) = it.peek() {
                        if c.is_digit(10) {
                            sub.push(it.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Num(
                        sub.iter().collect::<String>().parse::<usize>().unwrap(),
                    ))
                }
                ' ' => {
                    // ignore spaces
                }
                _ => panic!(format!("unexpected char {}", c)),
            },
            None => break,
        }
    }
    tokens
}

fn parse_expr(expr: &str) -> Vec<Token> {
    let mut it = expr.chars().peekable();
    parse(&mut it)
}

fn eval1(expr_str: &str) -> usize {
    let tokens = parse_expr(expr_str);

    fn eval_tokens(t: &[Token]) -> usize {
        fn as_num(t: &Token) -> usize {
            match t {
                Token::Num(n) => *n,
                Token::Expr(e) => eval_tokens(&e),
                _ => panic!(format!("expected number, instead saw token {:?}", t)),
            }
        }
        if t.len() == 1 {
            return as_num(&t[0]);
        }
        let left = as_num(&t[0]);
        let op = &t[1];
        let right = as_num(&t[2]);
        let val = match op {
            Token::Plus => left + right,
            Token::Times => left * right,
            _ => panic!(format!("unexpected op {:?}", op)),
        };
        if t.len() > 3 {
            let mut next = vec![Token::Num(val)];
            for x in &t[3..] {
                next.push(x.clone());
            }
            eval_tokens(&next)
        } else {
            val
        }
    }

    eval_tokens(&tokens)
}

fn eval2(expr_str: &str) -> usize {
    let tokens = parse_expr(expr_str);

    fn eval_tokens(t: &[Token]) -> usize {
        fn as_num(t: &Token) -> usize {
            match t {
                Token::Num(n) => *n,
                Token::Expr(e) => eval_tokens(&e),
                _ => panic!(format!("expected number, instead saw token {:?}", t)),
            }
        }
        if t.len() == 1 {
            return as_num(&t[0]);
        }
        let left = as_num(&t[0]);
        let op = &t[1];
        match op {
            Token::Plus => {
                let right = as_num(&t[2]);
                let val = left + right;
                if t.len() > 3 {
                    let mut next = vec![Token::Num(val)];
                    for x in &t[3..] {
                        next.push(x.clone());
                    }
                    eval_tokens(&next)
                } else {
                    val
                }
            }
            Token::Times => left * eval_tokens(&t[2..]),
            _ => panic!(format!("unexpected op {:?}", op)),
        }
    }

    eval_tokens(&tokens)
}

fn part1(s: &str) -> usize {
    s.split('\n').map(eval1).sum()
}

fn part2(s: &str) -> usize {
    s.split('\n').map(eval2).sum()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(eval1("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(eval1("2 * 3 + (4 * 5)"), 26);
    assert_eq!(eval1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(eval1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(
        eval1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
}

#[test]
fn test_part2() {
    assert_eq!(eval2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(eval2("2 * 3 + (4 * 5)"), 46);
    assert_eq!(eval2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(eval2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(
        eval2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}
