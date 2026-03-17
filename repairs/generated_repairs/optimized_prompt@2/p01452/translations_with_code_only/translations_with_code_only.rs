use std::io::{self, BufRead};

const N: i64 = 1_000_000_007;

struct Memo {
    data: Vec<i64>,
}

impl Memo {
    fn new(size: usize) -> Self {
        let mut data = vec![0; size];
        data[0] = 1; // Set memo[0] to 1
        Memo { data }
    }

    fn init_table(&mut self, n: usize) {
        for i in 1..=n {
            self.data[i] = (i as i64 * self.data[i - 1]) % N;
        }
    }

    fn get(&self, index: usize) -> i64 {
        self.data[index]
    }
}

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b != 0 {
        let (d, x, y) = extgcd(b, a % b);
        let new_y = y - (a / b) * x;
        (d, new_y, x)
    } else {
        (a, 1, 0)
    }
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (d, x, _) = extgcd(a, m);
    if d != 1 {
        panic!("Inverse does not exist");
    }
    (m + x % m) % m
}

fn mod_cmb(memo: &Memo, n: i64, r: i64, m: i64) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }

    let a1 = memo.get(n as usize);
    let a2 = memo.get(r as usize);
    let a3 = memo.get((n - r) as usize);

    (a1 * mod_inv((a2 * a3) % m, m)) % m
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let x: i64 = parts.next().unwrap().parse().unwrap();
    let y: i64 = parts.next().unwrap().parse().unwrap();
    let k: i64 = parts.next().unwrap().parse().unwrap();

    let mut memo = Memo::new((x + y + 2 * k) as usize);
    memo.init_table((x + y + 2 * k) as usize);
    let mut ans = 0;

    for i in 0..=k {
        let xgr = (mod_cmb(&memo, x + 2 * (k - i), k - i, N) - mod_cmb(&memo, x + 2 * (k - i), k - i - 1, N) + N) % N;
        let ygr = (mod_cmb(&memo, y + 2 * i, i, N) - mod_cmb(&memo, y + 2 * i, i - 1, N) + N) % N;

        ans = (ans + (xgr * ygr % N) * mod_cmb(&memo, x + y + 2 * k, y + 2 * i, N) % N) % N;
    }
    
    println!("{}", (ans + N) % N);
}
