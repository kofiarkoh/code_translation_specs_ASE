use std::io::{self, Read};

const MAX: usize = 1_000_001;
const MOD: i64 = 1_000_000_007;

fn com_init(fac: &mut [i64], finv: &mut [i64], inv: &mut [i64]) {
    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;
    inv[1] = 1;

    for i in 2..MAX {
        let ii = i as i64;
        fac[i] = fac[i - 1] * ii % MOD;

        // inv[i] = MOD - inv[MOD%i] * (MOD / i) % MOD;
        let idx = (MOD % ii) as usize;
        inv[i] = MOD - (inv[idx] * (MOD / ii) % MOD);

        finv[i] = finv[i - 1] * inv[i] % MOD;
    }
}

fn com(n: i32, k: i32, fac: &[i64], finv: &[i64]) -> i64 {
    if n < k {
        return 0;
    }
    if n < 0 || k < 0 {
        return 0;
    }
    let n = n as usize;
    let k = k as usize;
    fac[n] * (finv[k] * finv[n - k] % MOD) % MOD
}

fn main() {
    // Read entire stdin, parse two ints like scanf("%d%d", &x, &y)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let x: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // if no input, do nothing (closest to C's unspecified behavior)
    };
    let y: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    if (x + y) % 3 != 0 {
        print!("0\n");
        return;
    }
    let n: i32 = (x + y) / 3;

    let x_ = x - n;
    let y_ = y - n;

    if x_ < 0 || y_ < 0 {
        print!("0\n");
        return;
    }

    let mut fac = vec![0i64; MAX];
    let mut finv = vec![0i64; MAX];
    let mut inv = vec![0i64; MAX];
    com_init(&mut fac, &mut finv, &mut inv);

    let ans = com(x_ + y_, x_, &fac, &finv) % MOD;
    print!("{ans}\n");
}
