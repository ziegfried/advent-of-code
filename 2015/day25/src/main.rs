const FIRST_CODE: u64 = 20151125;
const FACTOR: u64 = 252533;
const MOD: u64 = 33554393;
const INPUT_ROW: (usize, usize) = (2947, 3029);

fn compute_code() -> u64 {
    let mut cur: u64 = FIRST_CODE;
    let mut k = 0;
    loop {
        for col in 0..(k) {
            let row = k - col - 1;
            if (row + 1, col + 1) == INPUT_ROW {
                return cur;
            }
            cur = (cur * FACTOR) % MOD;
        }
        k += 1;
    }
    panic!("nah");
}

fn main() {
    dbg!(compute_code());
}
