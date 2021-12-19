#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate regex;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    lazy_static! {
        static ref LINE_RE: regex::Regex = regex::Regex::new(r"(.+?) to (.+?) = (\d+)").unwrap();
    }

    let mut locations = HashSet::new();
    let mut distances = HashMap::new();

    for line in include_str!("../in.txt").split('\n') {
        let m = LINE_RE.captures(line).unwrap();
        let from = String::from(&m[1]);
        let to = String::from(&m[2]);
        locations.insert(from.clone());
        locations.insert(to.clone());
        let distance = (&m[3]).parse::<usize>().unwrap();
        distances.insert(format!("{}-{}", from, to), distance);
        distances.insert(format!("{}-{}", to, from), distance);
    }

    let loc_count: usize = locations.len();
    let mut min_dist: usize = usize::MAX;
    let mut max_dist: usize = 0;

    for route in (&locations).into_iter().permutations(loc_count) {
        let dist: usize = route
            .windows(2)
            .map(|w| {
                let k = format!("{}-{}", w[0], w[1]);
                let dist = distances.get(&k);
                match dist {
                    Some(v) => v,
                    None => panic!("meh"),
                }
            })
            .sum();
        if dist < min_dist {
            min_dist = dist;
        }
        if dist > max_dist {
            max_dist = dist;
        }
    }

    println!("Part 1: {}", min_dist);
    println!("Part 2: {}", max_dist);
}
