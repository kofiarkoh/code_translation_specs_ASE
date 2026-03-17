use std::io::{self, BufRead};

const N: i64 = 1_000_000_007;
const MAX_SIZE: usize = 220_001;

struct Memo {
    values: [i64; MAX_SIZE],
}

impl Memo {
    fn new() -> Self {
        let mut values = [0; MAX_SIZE];
        values[0] = 1; // Initialize memo[0] to 1
        Memo { values }
    }

    fn init_table(&mut self, n: usize) {
        self.values[1] = 1;
        for i in 2..=n {
            self.values[i] = (i as i64 * self.values[i - 1]) % N;
        }
    }

    fn get(&self, index: usize) -> i64 {
        self.values[index]
    }
}

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x1, y1) = extgcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (d, x, y)
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (_, x, _) = extgcd(a, m);
    (m + x % m) % m
}

fn mod_cmb(memo: &Memo, n: i64, r: i64) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let a1 = memo.get(n as usize);
    let a2 = memo.get(r as usize);
    let a3 = memo.get((n - r) as usize);
    (a1 * mod_inv((a2 * a3) % N, N)) % N
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut parts = input.split_whitespace();
    let x: i64 = parts.next().unwrap().parse().unwrap();
    let y: i64 = parts.next().unwrap().parse().unwrap();
    let k: i64 = parts.next().unwrap().parse().unwrap();

    let mut memo = Memo::new();
    memo.init_table((x + y + 2 * k) as usize);
    
    let mut ans = 0;

    for i in 0..=k {
        let xgr = (mod_cmb(&memo, x + 2 * (k - i), k - i) - mod_cmb(&memo, x + 2 * (k - i), k - i - 1) + N) % N;
        let ygr = (mod_cmb(&memo, y + 2 * i, i) - mod_cmb(&memo, y + 2 * i, i - 1) + N) % N;

        ans = (ans + (xgr * ygr % N) * mod_cmb(&memo, x + y + 2 * k, y + 2 * i) % N) % N;
    }

    println!("{}", (ans + N) % N);
}
