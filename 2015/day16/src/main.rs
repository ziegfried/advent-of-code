use std::collections::HashMap;

fn parse_map(s: &str, sep: &'static str) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    for part in s.split(sep) {
        let mut kv = part.trim().split(": ");
        let key = String::from(kv.next().unwrap());
        let value = kv.next().unwrap().parse::<usize>().unwrap();
        map.insert(key, value);
    }
    map
}

fn parse_aunt(s: &str) -> (String, HashMap<String, usize>) {
    let first_sep = s.find(':').unwrap();
    (
        s[0..first_sep].to_string(),
        parse_map(&s[(first_sep + 1)..], ", "),
    )
}

fn main() {
    let compounds = parse_map(include_str!("compounds.txt"), "\n");
    let aunts = include_str!("in.txt")
        .lines()
        .map(parse_aunt)
        .collect::<Vec<_>>();

    for (aunt, props) in aunts.clone() {
        if props.iter().all(|(k, v)| compounds.get(k) == Some(v)) {
            println!("Part 1: {}", aunt);
            break;
        }
    }

    for (aunt, props) in aunts {
        if props.iter().all(|(k, v)| {
            let val = compounds.get(k).unwrap();
            match k.as_str() {
                "cats" | "trees" => v > val,
                "pomeranians" | "goldfish" => v < val,
                _ => v == val,
            }
        }) {
            println!("Part 2: {}", aunt);
        }
    }
}
