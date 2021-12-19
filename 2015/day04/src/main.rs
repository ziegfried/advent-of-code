extern crate md5;

fn solve(prefix: &str) -> (u64, String) {
    let mut i: u64 = 0;
    loop {
        let digest = md5::compute(format!("bgvyzdsv{}", i));
        let hash = format!("{:x}", digest);
        if hash.starts_with(prefix) {
            return (i, hash);
        }
        i += 1;
    }
}

fn main() {
    println!("Part 1: {}", solve("00000").0);
    println!("Part 2: {}", solve("000000").0);
}
