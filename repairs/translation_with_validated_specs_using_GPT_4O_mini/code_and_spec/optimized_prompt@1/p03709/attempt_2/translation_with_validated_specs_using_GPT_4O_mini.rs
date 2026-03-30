use std::cmp::{max, min};
use std::io::{self, BufRead};

const MOD: u64 = 1_000_000_007;
const N_MAX: usize = 1_048_576;

#[derive(Clone, Copy)]
struct Hwll {
    a: i64,
    b: i64,
}

fn gcd(x: u64, y: u64) -> u64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn bitpow(a: u64, x: u64, modulo: u64) -> u64 {
    let mut result = 1;
    let mut a = a;
    let mut x = x;
    while x > 0 {
        if x & 1 == 1 {
            result = (result * a) % modulo;
        }
        x >>= 1;
        a = (a * a) % modulo;
    }
    result
}

fn solve(n: usize, a: &mut [i64], b: &mut [i64]) -> (String, i64) {
    let mut xy: Vec<Hwll> = Vec::with_capacity(n);
    for i in 0..n {
        xy.push(Hwll { a: a[i], b: b[i] });
    }
    xy.sort_by(|l, r| {
        if l.a != r.a {
            l.a.cmp(&r.a)
        } else {
            l.b.cmp(&r.b)
        }
    });

    for i in 0..n {
        a[i] = xy[i].a;
        b[i] = xy[i].b;
    }

    let mut c = vec![0; n];
    let mut d = vec![0; n];
    for i in 0..n {
        c[i] = max(if i > 0 { c[i - 1] } else { 0 }, b[i]);
    }
    for i in (0..n).rev() {
        d[i] = min(if i < n - 1 { d[i + 1] } else { i64::MAX }, b[i]);
    }

    let mut e = vec![-1; n + 2];
    let mut result = 0;
    let mut ss = vec![vec![0; 4]; n + 1];
    let inv2 = (MOD + 1) / 2;

    for i in 0..n {
        let (mut ok, mut ng) = (-1, i as i64);
        if i == 0 || c[i - 1] <= b[i] {
            xy[i].a = i as i64;
        } else {
            while ok + 1 < ng {
                let med = (ok + ng) / 2;
                if c[med as usize] > b[i] {
                    ng = med;
                } else {
                    ok = med;
                }
            }
            xy[i].a = ng;
        }

        ok = n as i64;
        ng = i as i64;
        if i == n - 1 || d[i + 1] >= b[i] {
            xy[i].b = i as i64;
        } else {
            while ng + 1 < ok {
                let med = (ok + ng) / 2;
                if d[med as usize] < b[i] {
                    ng = med;
                } else {
                    ok = med;
                }
            }
            xy[i].b = ng;
        }
    }

    for i in 0..n {
        xy[i].a += 1;
        xy[i].b += 1;
        e[xy[i].b as usize] = xy[i].a;
    }

    let mut dp = vec![vec![0; 4]; n + 1];
    dp[0][0] = bitpow(2, n as u64, MOD);
    dp[0][2] = bitpow(2, n as u64, MOD);
    for i in 1..=n {
        if e[i as usize] >= 0 {
            let l = e[i as usize] as usize;
            let r = i as usize;
            for ki in 0..2 {
                dp[i][ki] = (ss[l - 1][1 - ki] * bitpow(inv2, (r - l + 1) as u64, MOD)) % MOD;
                dp[i][ki] += (MOD + ss[r - 1][3 - ki] - ss[l - 1][3 - ki]) * bitpow(inv2, r as u64, MOD) % MOD;
                dp[i][ki] %= MOD;
                dp[i][2 + ki] = (dp[i][ki] * bitpow(2, i as u64, MOD)) % MOD;
            }
        }
        for ki in 0..4 {
            ss[i][ki] = (ss[i - 1][ki] + dp[i][ki]) % MOD;
        }
    }

    result = (MOD + ss[n][0] - ss[n][1]) % MOD;

    if result > 0 {
        (String::from("Yes"), result as i64)
    } else {
        (String::from("No"), result as i64)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    let n: usize = inputs.next().unwrap().parse().unwrap();
    let _m: usize = inputs.next().unwrap().parse().unwrap();

    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        a[i] = nums.next().unwrap().parse().unwrap();
        b[i] = nums.next().unwrap().parse().unwrap();
    }

    let (result_str, result_value) = solve(n, &mut a, &mut b);
    println!("{}", result_str);
    println!("{}", result_value);
}
