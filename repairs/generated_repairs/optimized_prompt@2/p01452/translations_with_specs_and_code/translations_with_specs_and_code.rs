const N: i64 = 1_000_000_007;

struct Memo {
    data: Vec<i64>,
}

impl Memo {
    fn new(size: usize) -> Self {
        let mut data = vec![0; size + 1]; // Allocate size + 1 to avoid index out of bounds
        data[1] = 1; // Initialize memo[1] to 1
        Memo { data }
    }

    fn init_table(&mut self, n: usize) {
        for i in 2..=n {
            self.data[i] = (i as i64 * self.data[i - 1]) % N;
        }
    }

    fn get(&self, index: usize) -> i64 {
        if index < self.data.len() {
            self.data[index]
        } else {
            0 // Return 0 for out of bounds access
        }
    }
}

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b != 0 {
        let (d, x1, y1) = extgcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (d, x, y)
    } else {
        (a, 1, 0)
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (gcd, x, _) = extgcd(a, m);
    if gcd != 1 {
        0 // Inverse doesn't exist
    } else {
        (m + x % m) % m
    }
}

fn mod_cmb(n: i64, r: i64, m: i64, memo: &Memo) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let a1 = memo.get(n as usize);
    let a2 = memo.get(r as usize);
    let a3 = memo.get((n - r) as usize);
    (a1 * mod_inv((a2 * a3) % m, m)) % m
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut parts = input.split_whitespace();
    let x: i64 = parts.next().unwrap().parse().unwrap();
    let y: i64 = parts.next().unwrap().parse().unwrap();
    let k: i64 = parts.next().unwrap().parse().unwrap();

    if x < 0 || y < 0 || k < 0 {
        println!("0");
        return;
    }

    let size = (x + y + 2 * k) as usize;
    let mut memo = Memo::new(size);
    memo.init_table(size);
    let mut ans = 0;

    for i in 0..=k {
        let xgr = (mod_cmb(x + 2 * (k - i), k - i, N, &memo) - mod_cmb(x + 2 * (k - i), k - i - 1, N, &memo) + N) % N;
        let ygr = (mod_cmb(y + 2 * i, i, N, &memo) - mod_cmb(y + 2 * i, i - 1, N, &memo) + N) % N;

        ans = (ans + ((xgr * ygr) % N) * mod_cmb(x + y + 2 * k, y + 2 * i, N, &memo)) % N;
    }
    println!("{}", (ans + N) % N);
}
