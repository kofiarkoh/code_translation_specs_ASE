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

fn bitpow(a: u64, mut x: u64, modulo: u64) -> u64 {
    let mut result = 1;
    while x > 0 {
        if x & 1 == 1 {
            result = (result * a) % modulo;
        }
        x >>= 1;
        a = (a * a) % modulo;
    }
    result
}

fn solve(n: usize, a: &mut Vec<i64>, b: &mut Vec<i64>) -> i32 {
    let mut xy: Vec<Hwll> = Vec::with_capacity(n);
    let mut c: Vec<i64> = vec![0; n];
    let mut d: Vec<i64> = vec![0; n];
    let mut dp: Vec<Vec<u64>> = vec![vec![0; 4]; n + 1];
    let mut ss: Vec<Vec<u64>> = vec![vec![0; 4]; n + 1];
    let mut e: Vec<i64> = vec![-1; n + 2];

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

    for i in 0..n {
        c[i] = if i == 0 { b[i] } else { c[i - 1].max(b[i]) };
    }
    for i in (0..n).rev() {
        d[i] = if i == n - 1 { i64::MAX } else { d[i + 1].min(b[i]) };
    }

    for i in 0..n {
        let (mut ok, mut ng) = if i == 0 || c[i - 1] <= b[i] {
            (0, 0)
        } else {
            (-1, i)
        };
        while ok + 1 < ng {
            let med = (ok + ng) / 2;
            if c[med] > b[i] {
                ng = med;
            } else {
                ok = med;
            }
        }
        xy[i].a = ng;

        let (mut ok, mut ng) = if i == n - 1 || d[i + 1] >= b[i] {
            (0, 0)
        } else {
            (n, i)
        };
        while ng + 1 < ok {
            let med = (ok + ng) / 2;
            if d[med] < b[i] {
                ng = med;
            } else {
                ok = med;
            }
        }
        xy[i].b = ng;
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

    dp[0][0] = bitpow(2, n as u64, MOD);
    dp[0][2] = dp[0][0];

    for i in 1..=n {
        if e[i] >= 0 {
            let l = e[i] as usize;
            let r = i;
            for ki in 0..2 {
                dp[i][ki] = (ss[l - 1][1 - ki] * bitpow((MOD + 1) / 2, (r - l + 1) as u64, MOD)) % MOD;
                dp[i][ki] += (MOD + ss[r - 1][3 - ki] - ss[l - 1][3 - ki]) * bitpow((MOD + 1) / 2, r as u64, MOD) % MOD;
                dp[i][ki] %= MOD;

                dp[i][2 + ki] = (dp[i][ki] * bitpow(2, i as u64, MOD)) % MOD;
            }
        }

        for ki in 0..4 {
            ss[i][ki] = (ss[i - 1][ki] + dp[i][ki]) % MOD;
        }
    }

    let result = (MOD + ss[n][0] - ss[n][1]) % MOD;

    println!("Yes");
    println!("{}", result);
    0
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i64> = vec![0; n];
    let mut b: Vec<i64> = vec![0; n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        a[i] = nums[0];
        b[i] = nums[1];
    }

    solve(n, &mut a, &mut b);
}
