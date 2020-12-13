fn iterate(s: String) -> String {
    let mut it = s.chars().peekable();
    let mut res = vec![];
    loop {
        if let Some(c) = it.next() {
            let mut count = 1;
            while it.peek() == Some(&c) {
                it.next();
                count += 1;
            }
            res.push(format!("{}{}", count, c));
        } else {
            break;
        }
    }

    res.join("")
}

fn main() {
    let mut s = "1321131112".to_string();
    for _ in 0..40 {
        s = iterate(s);
    }
    println!("Part 1: {}", s.len());
    for _ in 0..10 {
        s = iterate(s);
    }
    println!("Part 2: {}", s.len());
}
