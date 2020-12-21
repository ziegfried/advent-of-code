#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse_line(s: &str) -> (Vec<String>, Vec<String>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+) \(contains (.+)\)").unwrap();
    }
    let m = RE.captures(s).unwrap();
    (
        m[1].split(' ').map(String::from).collect::<Vec<String>>(),
        m[2].split(", ").map(String::from).collect::<Vec<String>>(),
    )
}

fn solve(s: &str) -> (usize, String) {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingredients: Vec<String> = vec![];
    let input = s.lines().map(parse_line);

    for (ingredients, allergens) in input.clone() {
        all_ingredients.extend_from_slice(&ingredients[..]);
        let i = ingredients.iter().cloned().collect::<HashSet<String>>();
        for a in allergens {
            if map.contains_key(&a) {
                let v = map.get_mut(&a).unwrap();
                *v = v
                    .intersection(&i.clone())
                    .cloned()
                    .collect::<HashSet<String>>();
            } else {
                map.insert(a, i.clone());
            }
        }
    }

    let inv = map
        .values()
        .flat_map(|v| v.iter().map(|s| s.clone()))
        .collect::<HashSet<String>>();

    let part1 = all_ingredients
        .iter()
        .filter(|i| !(&inv).contains(i.clone()))
        .count();

    let mut known = HashMap::new();
    loop {
        let mut found = vec![];
        for (allergen, possible_ingredients) in map.clone() {
            if possible_ingredients.len() == 1 {
                let ingredient = possible_ingredients.iter().next().unwrap();
                found.push((allergen.clone(), ingredient.clone()));
                known.insert(allergen.clone(), ingredient.clone());
            }
        }
        if found.len() > 0 {
            for (a, i) in found {
                map.remove(&a);
                for val in map.values_mut() {
                    val.remove(&i);
                }
            }
        } else {
            break;
        }
    }

    let mut allergens = known.keys().cloned().collect::<Vec<String>>();
    allergens.sort();

    let part2 = allergens
        .iter()
        .map(|a| known.get(a).unwrap().clone())
        .collect::<Vec<String>>()
        .join(",");

    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(include_str!("in.txt"));
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[test]
fn test_input() {
    assert_eq!(
        solve(include_str!("test.txt")),
        (5, String::from("mxmxvkd,sqjhc,fvjkl"))
    )
}
