use itertools::Itertools;

fn main() {
    let preamble_len = 25;
    let inputs = include_str!("../in.txt")
        .split('\n')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut magic_number = 0;

    for i in (preamble_len)..(inputs.len()) {
        let n = inputs[i];
        let sum_found = (&inputs[(i - preamble_len)..(i)])
            .to_vec()
            .into_iter()
            .permutations(2)
            .any(|p| p[0] + p[1] == n);
        if !sum_found {
            println!("Part 1: {}", n);
            magic_number = n;
            break;
        }
    }

    let len = inputs.len();
    for i in 0..(len) {
        let mut sum = 0;
        for j in 0..(len - i) {
            sum += inputs[i + j];
            if sum == magic_number {
                let range: Vec<u64> = (&inputs[(i)..(i + j)]).to_vec();
                let min = (&range).into_iter().min().unwrap();
                let max = (&range).into_iter().max().unwrap();
                println!("Part 2: {}", min + max);
                return;
            }
            if sum > magic_number {
                break;
            }
        }
    }
}
