use std::io::{self, BufRead};

const MOD: u64 = 1_000_000_007;
const N_MAX: usize = 1_048_576;

#[derive(Copy, Clone)]
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
        x /= 2;
        a = (a * a) % modulo;
    }
    result
}

fn solve(n: usize, a: &[i64], b: &[i64]) -> (String, u64) {
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

    let mut c = vec![0; n];
    let mut d = vec![0; n];
    for i in 0..n {
        c[i] = if i == 0 { 0 } else { c[i - 1].max(b[i]) };
    }
    for i in (0..n).rev() {
        d[i] = if i == n - 1 { i64::MAX } else { d[i + 1].min(b[i]) };
    }

    let mut e = vec![-1; n + 2];
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

    xy.sort_by(|l, r| {
        if l.a != r.a {
            l.a.cmp(&r.a)
        } else {
            l.b.cmp(&r.b)
        }
    });

    for i in 0..n {
        xy[i].a += 1;
        xy[i].b += 1;
        e[xy[i].b as usize] = xy[i].a;
    }

    let mut dp = vec![vec![0; 4]; n + 1];
    let mut ss = vec![vec![0; 4]; n + 1];
    dp[0][0] = bitpow(2, n as u64, MOD);
    dp[0][2] = bitpow(2, n as u64, MOD);
    for i in 1..=n {
        if e[i as usize] >= 0 {
            let l = e[i as usize];
            let r = i as i64;
            for ki in 0..2 {
                dp[i as usize][ki] = (ss[l as usize - 1][1 - ki] * bitpow((MOD + 1) / 2, (r - l + 1) as u64, MOD)) % MOD;
                dp[i as usize][ki] += (MOD + ss[r as usize - 1][3 - ki] - ss[l as usize - 1][3 - ki]) * bitpow((MOD + 1) / 2, r as u64, MOD) % MOD;
                dp[i as usize][ki] %= MOD;
                dp[i as usize][2 + ki] = dp[i as usize][ki] * bitpow(2, i as u64, MOD) % MOD;
            }
        }
        for ki in 0..4 {
            ss[i as usize][ki] = (ss[i as usize - 1][ki] + dp[i as usize][ki]) % MOD;
        }
    }

    let result = (MOD + ss[n][0] - ss[n][1]) % MOD;
    if result > 0 {
        ("Yes".to_string(), result)
    } else {
        ("No".to_string(), result)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        a[i] = nums[0];
        b[i] = nums[1];
    }

    let (output, result) = solve(n, &a, &b);
    println!("{}", output);
    println!("{}", result);
}
