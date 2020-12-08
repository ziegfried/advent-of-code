use std::collections::HashSet;

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for s in include_str!("../in.txt").split("\n\n") {
        let mut answers = HashSet::new();
        for ans in s.split('\n') {
            for c in ans.chars() {
                answers.insert(c);
            }
        }
        part1 += answers.len();

        let mut common_answers = answers.clone();
        for ans in s.split('\n') {
            if ans != "" {
                let mut local = HashSet::new();
                for c in ans.chars() {
                    local.insert(c);
                }
                for c in answers.iter() {
                    if !local.contains(c) {
                        common_answers.remove(c);
                    }
                }
            }
        }
        part2 += common_answers.len();
    }

    dbg!(part1, part2);
}
