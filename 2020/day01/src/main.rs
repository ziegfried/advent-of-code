fn main() {
    let entries = include_str!("../in.txt")
        .split('\n')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let count = entries.len();

    println!("Part 1");
    'loop1: for i in 0..(count) {
        for j in 0..(count) {
            let x = entries[i];
            let y = entries[j];
            if x != y && x + y == 2020 {
                println!("{} + {} = {}", x, y, x + y);
                println!("{} * {} = {}", x, y, x * y);
                break 'loop1;
            }
        }
    }

    println!("Part 2:");
    'loop2: for i in 0..(count) {
        for j in 0..(count) {
            for k in 0..(count) {
                let x = entries[i];
                let y = entries[j];
                let z = entries[k];
                if x + y + z == 2020 {
                    println!("{} + {} + {} = {}", x, y, z, x + y + z);
                    println!("{} * {} * {} = {}", x, y, z, x * y * z);
                    break 'loop2;
                }
            }
        }
    }
}
