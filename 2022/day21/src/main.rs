// Problem: https://adventofcode.com/2022/day/21

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Monkey {
    Number(i64),
    Calc(String, String, char),
}

fn parse_monkey(s: &str) -> Monkey {
    match s.parse::<i64>() {
        Ok(v) => Monkey::Number(v),
        Err(_) => {
            let (left, op, right) = sscanf::sscanf!(s, "{String} {char} {String}").unwrap();
            Monkey::Calc(left, right, op)
        }
    }
}

fn parse(input: &str) -> HashMap<String, Monkey> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (monkey, formula) = line.split_once(": ").unwrap();
            (monkey.to_string(), parse_monkey(formula))
        })
        .collect()
}

fn resolve(monkey: &String, monkeys: &HashMap<String, Monkey>) -> i64 {
    match monkeys.get(monkey).unwrap() {
        Monkey::Number(n) => *n,
        Monkey::Calc(l, r, op) => {
            let l = resolve(l, monkeys);
            let r = resolve(r, monkeys);
            match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                _ => panic!("invalid op {}", op),
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    let monkeys = parse(input);
    resolve(&"root".to_string(), &monkeys)
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 152);
}

// ---------------------------------

fn reverse_operation(left: &Monkey, right: &Monkey, op: char, result: i64) -> i64 {
    let (n, fixed_is_left) = match (left, right) {
        (Monkey::Number(n), Monkey::Calc(_, _, _)) => (*n, true),
        (Monkey::Calc(_, _, _), Monkey::Number(n)) => (*n, false),
        _ => panic!(),
    };
    match op {
        '+' => result - n,
        '-' => {
            if fixed_is_left {
                n - result
            } else {
                result + n
            }
        }
        '*' => result / n,
        '/' => {
            if fixed_is_left {
                n / result
            } else {
                n * result
            }
        }
        _ => panic!("invalid op"),
    }
}

#[test]
fn test_reverse_reverse_operation() {
    let dyn_val = Monkey::Calc("foobar".to_string(), "dingdong".to_string(), '+');
    let five = Monkey::Number(5);
    let twelve = Monkey::Number(12);

    assert_eq!(reverse_operation(&five, &dyn_val, '+', 12), 7);
    assert_eq!(reverse_operation(&dyn_val, &Monkey::Number(5), '+', 12), 7);

    assert_eq!(reverse_operation(&five, &dyn_val, '-', 2), 3);
    assert_eq!(reverse_operation(&dyn_val, &five, '-', 10), 15);

    assert_eq!(reverse_operation(&five, &dyn_val, '*', 50), 10);
    assert_eq!(reverse_operation(&dyn_val, &five, '*', 50), 10);

    assert_eq!(reverse_operation(&twelve, &dyn_val, '/', 3), 4);
    assert_eq!(reverse_operation(&dyn_val, &five, '/', 10), 50);
}

fn has_target_dep(id: &String, target: &String, monkeys: &HashMap<String, Monkey>) -> bool {
    match monkeys.get(id).unwrap() {
        Monkey::Number(_) => false,
        Monkey::Calc(left, right, _) => {
            if left == target || right == target {
                true
            } else {
                has_target_dep(left, target, monkeys) || has_target_dep(right, target, monkeys)
            }
        }
    }
}

fn reverse_resolve(
    id: &String,
    me: &String,
    needed_val: i64,
    monkeys: &HashMap<String, Monkey>,
) -> i64 {
    if let Monkey::Calc(left_id, right_id, op) = monkeys.get(id).unwrap() {
        let left = monkeys.get(left_id).unwrap();
        let right = monkeys.get(right_id).unwrap();

        if left_id == me {
            let resolved_right = resolve(right_id, monkeys);
            let result = reverse_operation(
                &Monkey::Calc(me.clone(), me.clone(), '#'),
                &Monkey::Number(resolved_right),
                *op,
                needed_val,
            );
            return result;
        }
        if right_id == me {
            let resolved_left = resolve(left_id, monkeys);
            let result = reverse_operation(
                &Monkey::Number(resolved_left),
                &Monkey::Calc(me.clone(), me.clone(), '#'),
                *op,
                needed_val,
            );
            return result;
        }

        match (left, right) {
            (Monkey::Number(_), Monkey::Calc(_, _, _)) => {
                let next_needed = reverse_operation(left, right, *op, needed_val);
                return reverse_resolve(right_id, me, next_needed, monkeys);
            }
            (Monkey::Calc(_, _, _), Monkey::Number(_)) => {
                let next_needed = reverse_operation(left, right, *op, needed_val);
                return reverse_resolve(left_id, me, next_needed, monkeys);
            }
            (Monkey::Calc(_, _, _), Monkey::Calc(_, _, _)) => {
                match (
                    has_target_dep(left_id, me, monkeys),
                    has_target_dep(right_id, me, monkeys),
                ) {
                    (true, true) => {
                        panic!("both sides are dyn");
                    }
                    (true, false) => {
                        let right_val = resolve(right_id, monkeys);
                        let next_needed =
                            reverse_operation(left, &Monkey::Number(right_val), *op, needed_val);
                        return reverse_resolve(left_id, me, next_needed, monkeys);
                    }
                    (false, true) => {
                        let left_val = resolve(left_id, monkeys);
                        let next_needed =
                            reverse_operation(&Monkey::Number(left_val), right, *op, needed_val);
                        return reverse_resolve(right_id, me, next_needed, monkeys);
                    }
                    (false, false) => {
                        panic!()
                    }
                }
            }
            (Monkey::Number(_), Monkey::Number(_)) => panic!(),
        }
    }
    unreachable!()
}

fn part2(input: &str) -> i64 {
    let monkeys = parse(input);
    let me = "humn".to_string();

    let (left, right) = match monkeys.get(&"root".to_string()).unwrap() {
        Monkey::Calc(l, r, _) => (l, r),
        _ => panic!(),
    };

    let (fixed_root, dyn_root) = match (
        has_target_dep(left, &me, &monkeys),
        has_target_dep(right, &me, &monkeys),
    ) {
        (true, false) => (right, left),
        (false, true) => (left, right),
        _ => panic!(),
    };

    let expected_result = resolve(fixed_root, &monkeys);
    reverse_resolve(dyn_root, &me, expected_result, &monkeys)
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 301);
}

// ---------------------------------

fn main() {
    println!("Part 1: {:?}", part1(include_str!("input.txt")));
    println!("Part 2: {:?}", part2(include_str!("input.txt")));
}
