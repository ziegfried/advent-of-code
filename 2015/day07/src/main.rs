#[macro_use]
extern crate lazy_static;
extern crate regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Value {
    Scalar(u16),
    Ref(String),
}

#[derive(Debug, PartialEq)]
enum Gate {
    Value(Value),
    Lshift(Value, Value),
    Rshift(Value, Value),
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
}

fn parse_value(valuestr: &str) -> Value {
    match valuestr.parse::<u16>() {
        Ok(v) => Value::Scalar(v),
        Err(_) => Value::Ref(String::from(valuestr)),
    }
}

fn parse_gate(gatestr: &str) -> Gate {
    lazy_static! {
        static ref BIN_EXPR_RE: regex::Regex =
            regex::Regex::new(r"(\w+) (AND|OR|LSHIFT|RSHIFT) (\w+)").unwrap();
        static ref UNARY_EXPR_RE: regex::Regex = regex::Regex::new(r"NOT (\w+)").unwrap();
        static ref VAL_EXPR_RE: regex::Regex = regex::Regex::new(r"(\w+)").unwrap();
    }

    if let Some(m) = BIN_EXPR_RE.captures(gatestr) {
        let left = parse_value(&m[1]);
        let right = parse_value(&m[3]);
        return match &m[2] {
            "AND" => Gate::And(left, right),
            "OR" => Gate::Or(left, right),
            "LSHIFT" => Gate::Lshift(left, right),
            "RSHIFT" => Gate::Rshift(left, right),
            _ => panic!("Invalid binary expr"),
        };
    }

    if let Some(m) = UNARY_EXPR_RE.captures(gatestr) {
        return Gate::Not(parse_value(&m[1]));
    }

    if let Some(m) = VAL_EXPR_RE.captures(gatestr) {
        return Gate::Value(parse_value(&m[1]));
    }

    panic!("Invalid expr");
}

fn parse_line(line: &str) -> (Gate, String) {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(.+?) -> (\w+)").unwrap();
    }
    let m = RE.captures(line).unwrap();
    return (parse_gate(&m[1]), String::from(&m[2]));
}

fn resolve(value: &Value, wires: &HashMap<String, Gate>, memo: &mut HashMap<String, u16>) -> u16 {
    println!("RESOLVE {:?}", value);
    match value {
        Value::Scalar(val) => *val,
        Value::Ref(name) => {
            if let Some(v) = memo.get(name) {
                *v
            } else {
                let v = eval(wires.get(name).unwrap(), wires, memo);
                memo.insert(name.clone(), v);
                v
            }
        }
    }
}

fn eval(gate: &Gate, wires: &HashMap<String, Gate>, memo: &mut HashMap<String, u16>) -> u16 {
    println!("EVAL {:?}", gate);
    let v = match gate {
        Gate::Value(v) => resolve(v, wires, memo),
        Gate::And(left, right) => resolve(left, wires, memo) & resolve(right, wires, memo),
        Gate::Or(left, right) => resolve(left, wires, memo) | resolve(right, wires, memo),
        Gate::Lshift(left, right) => resolve(left, wires, memo) << resolve(right, wires, memo),
        Gate::Rshift(left, right) => resolve(left, wires, memo) >> resolve(right, wires, memo),
        Gate::Not(value) => resolve(value, wires, memo) ^ 0xFFFF,
    };
    println!("EVAL {:?} -> {}", gate, v);
    return v;
}

fn main() {
    let mut wires = HashMap::new();

    for line in include_str!("../in.txt").split('\n') {
        let (gate, wire) = parse_line(line);
        wires.insert(wire, gate);
    }

    // step 2
    wires.insert("b".to_string(), Gate::Value(Value::Scalar(956)));

    println!("{:?}", &wires);
    let mut memo = HashMap::new();
    println!("{:?}", eval(wires.get("a").unwrap(), &wires, &mut memo));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value("123"), Value::Scalar(123),);
        assert_eq!(parse_value("a"), Value::Ref(String::from("a")),);
        assert_eq!(parse_value("li"), Value::Ref(String::from("li")),);
    }

    #[test]
    fn test_parse_gate() {
        assert_eq!(
            parse_gate("1 AND 2"),
            Gate::And(Value::Scalar(1), Value::Scalar(2)),
        );
        assert_eq!(
            parse_gate("a LSHIFT b"),
            Gate::Lshift(Value::Ref("a".to_string()), Value::Ref("b".to_string())),
        );
        assert_eq!(parse_gate("NOT 1"), Gate::Not(Value::Scalar(1)),);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("as RSHIFT 3 -> au"),
            (
                Gate::Rshift(Value::Ref("as".to_string()), Value::Scalar(3)),
                "au".to_string()
            ),
        );
    }
}
