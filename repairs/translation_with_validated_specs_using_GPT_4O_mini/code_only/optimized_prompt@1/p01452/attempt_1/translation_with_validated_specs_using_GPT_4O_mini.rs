use std::io::{self, Write};

const N: i64 = 1000000007;

struct Memo {
    data: Vec<i64>,
}

impl Memo {
    fn new(size: usize) -> Self {
        let mut data = vec![0; size];
        data[1] = 1; // Initialize memo[1] to 1
        Memo { data }
    }

    fn init_table(&mut self, n: i64) {
        for i in 2..=n {
            self.data[i as usize] = (i * self.data[(i - 1) as usize]) % N;
        }
    }

    fn get(&self, index: i64) -> i64 {
        self.data[index as usize]
    }
}

fn extgcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let d = if b != 0 {
        let d = extgcd(b, a % b, y, x);
        *y -= a / b * *x;
        d
    } else {
        *x = 1;
        *y = 0;
        a
    };
    d
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let mut x = 0;
    let mut y = 0;
    extgcd(a, m, &mut x, &mut y);
    (m + x % m) % m
}

fn mod_cmb(n: i64, r: i64, m: i64, memo: &Memo) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let a1 = memo.get(n);
    let a2 = memo.get(r);
    let a3 = memo.get(n - r);
    (a1 * mod_inv((a2 * a3) % m, m) % m) % m
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x: i64 = iter.next().unwrap().parse().unwrap();
    let y: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    let mut memo = Memo::new((x + y + 2 * k + 1) as usize);
    memo.init_table(x + y + 2 * k);
    memo.data[0] = 1; // Set memo[0] to 1

    let mut ans = 0;

    for i in 0..=k {
        let xgr = (mod_cmb(x + 2 * (k - i), k - i, N, &memo) - mod_cmb(x + 2 * (k - i), k - i - 1, N, &memo) + N) % N;
        let ygr = (mod_cmb(y + 2 * i, i, N, &memo) - mod_cmb(y + 2 * i, i - 1, N, &memo) + N) % N;

        ans = (ans + ((xgr * ygr) % N) * mod_cmb(x + y + 2 * k, y + 2 * i, N, &memo) % N) % N;
    }

    println!("{}", (ans + N) % N);
}
