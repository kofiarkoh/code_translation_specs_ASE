use std::io::{self, Read};

const N: i64 = 1_000_000_007;

fn inittable(memo: &mut [i64], n: i64) {
    memo[1] = 1;
    for i in 2..=n {
        let idx = i as usize;
        memo[idx] = (i * memo[(i - 1) as usize]) % N;
    }
}

fn extgcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    let mut d = a;
    if b != 0 {
        // Note: mirrors the C recursion extgcd(b, a%b, y, x)
        d = extgcd(b, a % b, y, x);
        *y -= (a / b) * (*x);
    } else {
        *x = 1;
        *y = 0;
    }
    d
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let mut x = 0i64;
    let mut y = 0i64;
    extgcd(a, m, &mut x, &mut y);
    (m + x % m) % m
}

fn mod_cmb(memo: &[i64], n: i64, r: i64, m: i64) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let a1 = memo[n as usize];
    let a2 = memo[r as usize];
    let a3 = memo[(n - r) as usize];
    a1 * mod_inv((a2 * a3) % m, m) % m
}

fn main() {
    // Read all input (scanf-like whitespace handling)
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let x: i32 = it.next().unwrap().parse().unwrap();
    let y: i32 = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse().unwrap();

    // C has a fixed global array memo[220001]; keep same size.
    let mut memo = vec![0i64; 220_001];

    let max_n = (x as i64) + (y as i64) + 2 * (k as i64);
    inittable(&mut memo, max_n);
    memo[0] = 1;

    let mut ans: i64 = 0;

    for i in 0..=k {
        let ii = i as i64;
        let kk = k as i64;
        let xx = x as i64;
        let yy = y as i64;

        let x_n = xx + 2 * (kk - ii);
        let x_r = kk - ii;
        let xgr = (mod_cmb(&memo, x_n, x_r, N) - mod_cmb(&memo, x_n, x_r - 1, N) + N) % N;

        let y_n = yy + 2 * ii;
        let y_r = ii;
        let ygr = (mod_cmb(&memo, y_n, y_r, N) - mod_cmb(&memo, y_n, y_r - 1, N) + N) % N;

        let comb = mod_cmb(&memo, (xx + yy + 2 * kk), (yy + 2 * ii), N);
        ans = (ans + ((xgr * ygr) % N) * comb) % N;
    }

    // C prints with "%d\n" even though ans is long long; emulate by casting to i32.
    let out = ((ans + N) % N) as i32;
    print!("{out}\n");
}
