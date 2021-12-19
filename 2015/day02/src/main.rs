use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./in.txt").expect("ERR");
    let reader = BufReader::new(f);
    let mut wrap = 0;
    let mut ribbon = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts = &line
                .splitn(3, "x")
                .map(|d| d.parse::<usize>().expect(""))
                .collect::<Vec<usize>>();
            if let [l, w, h] = parts[..] {
                let slack = cmp::min(cmp::min(l * w, w * h), h * l);
                wrap += 2 * l * w + 2 * w * h + 2 * h * l + slack;
            }
            let mut sorted = parts.clone();
            sorted.sort();
            if let [a, b, _c] = sorted[..] {
                ribbon += 2 * a + 2 * b;
            }
            if let [l, w, h] = parts[..] {
                ribbon += l * w * h;
            }
        }
    }
    println!("Part 1: {}", wrap);
    println!("Part 2: {}", ribbon);
}
