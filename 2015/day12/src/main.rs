use serde_json::Value;

fn numeric_value(val: &Value) -> i64 {
    match val {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(v) => v.iter().map(numeric_value).sum(),
        Value::Object(v) => v.values().map(numeric_value).sum(),
    }
}

fn part1(s: &str) -> i64 {
    let json = serde_json::from_str(s).unwrap();
    numeric_value(&json)
}

fn numeric_value_without_red(val: &Value) -> i64 {
    match val {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(v) => v.iter().map(numeric_value_without_red).sum(),
        Value::Object(v) => {
            if v.values().any(|v| v == &Value::String("red".to_string())) {
                0
            } else {
                v.values().map(numeric_value_without_red).sum()
            }
        }
    }
}

fn part2(s: &str) -> i64 {
    let json = serde_json::from_str(s).unwrap();
    numeric_value_without_red(&json)
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(part1("[[[3]]]"), 3);
        assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(part1(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(part1(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(part1("[]"), 0);
        assert_eq!(part1("{}"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("[1,2,3]"), 6);
        assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(part2(r#"[1,"red",5]"#), 6);
    }
}
