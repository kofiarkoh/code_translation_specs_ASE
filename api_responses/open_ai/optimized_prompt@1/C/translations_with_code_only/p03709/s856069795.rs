use std::cmp::{max, min};
use std::io::{self, BufRead};
use std::mem;

const BIG: u64 = 2000000007;
const VERYBIG: i64 = 20000000000000007;
const MOD: u64 = 1000000007;
const N_MAX: usize = 1048576;

#[derive(Clone, Copy)]
struct Hwll {
    a: i64,
    b: i64,
}

#[derive(Clone, Copy)]
struct Hwllc {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Clone, Copy)]
struct Linell {
    a: Hwll,
    b: Hwll,
}

fn umin(x: u64, y: u64) -> u64 {
    if x < y { x } else { y }
}

fn umax(x: u64, y: u64) -> u64 {
    if x > y { x } else { y }
}

fn smin(x: i64, y: i64) -> i64 {
    if x < y { x } else { y }
}

fn smax(x: i64, y: i64) -> i64 {
    if x > y { x } else { y }
}

fn gcd(x: u64, y: u64) -> u64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn bitpow(a: u64, mut x: u64, modulo: u64) -> u64 {
    let mut result = 1;
    while x > 0 {
        if x & 1 == 1 {
            result = (result * a) % modulo;
        }
        x /= 2;
        a = (a * a) % modulo;
    }
    result
}

fn divide(a: u64, b: u64, modulo: u64) -> u64 {
    (a * bitpow(b, modulo - 2, modulo)) % modulo
}

fn udiff(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { b - a }
}

fn sdiff(a: i64, b: i64) -> i64 {
    if a >= b { a - b } else { b - a }
}

fn bitcount(mut n: u64) -> i32 {
    let mut result = 0;
    while n > 0 {
        if n & 1 == 1 {
            result += 1;
        }
        n /= 2;
    }
    result
}

fn isinrange(left: i64, x: i64, right: i64) -> bool {
    left <= x && x <= right
}

fn isinrange_soft(left: i64, x: i64, right: i64) -> bool {
    (left <= x && x <= right) || (left >= x && x >= right)
}

fn sw(l: &mut i64, r: &mut i64) {
    mem::swap(l, r);
}

fn ncr(n: i64, r: i64, m: u64, frac: &[u64], invf: &[u64]) -> u64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    frac[n as usize] * (invf[r as usize] * invf[(n - r) as usize] % m) % m
}

fn solve(n: usize, a: &mut [i64], b: &mut [i64]) -> i64 {
    let mut xy: Vec<Hwll> = vec![Hwll { a: 0, b: 0 }; n];
    let mut c: Vec<i64> = vec![0; n];
    let mut d: Vec<i64> = vec![0; n];
    let mut e: Vec<i64> = vec![-1; n + 2];
    let mut dp: Vec<Vec<u64>> = vec![vec![0; 4]; n + 1];
    let mut ss: Vec<Vec<u64>> = vec![vec![0; 4]; n + 1];

    let inv2 = (MOD + 1) / 2;

    for i in (0..n).rev() {
        xy[i] = Hwll { a: a[i], b: b[i] };
    }
    xy.sort_by(|l, r| {
        if l.a < r.a {
            std::cmp::Ordering::Less
        } else if l.a > r.a {
            std::cmp::Ordering::Greater
        } else {
            l.b.cmp(&r.b)
        }
    });

    for i in 0..n {
        a[i] = xy[i].a;
        b[i] = xy[i].b;
    }

    for i in 0..n {
        c[i] = smax(if i > 0 { c[i - 1] } else { 0 }, b[i]);
    }
    for i in (0..n).rev() {
        d[i] = smin(if i < n - 1 { d[i + 1] } else { VERYBIG }, b[i]);
    }

    for i in 0..n {
        let (mut ok, mut ng) = if i == 0 || c[i - 1] <= b[i] {
            (0, 0)
        } else {
            (-1, i as i64)
        };

        if ok == -1 {
            while ok + 1 < ng {
                let med = (ok + ng) / 2;
                if c[med as usize] > b[i] {
                    ng = med;
                } else {
                    ok = med;
                }
            }
            xy[i].a = ng;
        } else {
            xy[i].a = i as i64;
        }

        if i == n - 1 || d[i + 1] >= b[i] {
            xy[i].b = i as i64;
        } else {
            ok = n as i64;
            ng = i as i64;
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

    xy.sort_by(|l, r| {
        if l.a < r.a {
            std::cmp::Ordering::Less
        } else if l.a > r.a {
            std::cmp::Ordering::Greater
        } else {
            l.b.cmp(&r.b)
        }
    });

    for i in 0..n {
        xy[i].a += 1;
        xy[i].b += 1;
        e[xy[i].b as usize] = xy[i].a;
    }

    dp[0][0] = ss[0][0] = bitpow(2, n as u64, MOD);
    dp[0][2] = ss[0][2] = bitpow(2, n as u64, MOD);

    for i in 1..=n {
        if e[i] >= 0 {
            let l = e[i as usize];
            let r = i as i64;
            for ki in 0..2 {
                dp[i][ki] = ss[(l - 1) as usize][1 - ki] * bitpow(inv2, r - l + 1, MOD) % MOD;
                dp[i][ki] += (MOD + ss[(r - 1) as usize][3 - ki] - ss[(l - 1) as usize][3 - ki]) * bitpow(inv2, r, MOD) % MOD;
                dp[i][ki] %= MOD;

                dp[i][2 + ki] = dp[i][ki] * bitpow(2, i as u64, MOD) % MOD;
            }
        }

        for ki in 0..4 {
            ss[i][ki] = (ss[i - 1][ki] + dp[i][ki]) % MOD;
        }
    }

    let result = (MOD + ss[n][0] - ss[n][1]) % MOD;

    println!("{}", result);
    0 // Return 0 for success
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        a[i] = nums[0];
        b[i] = nums[1];
    }

    solve(n, &mut a, &mut b);
}
