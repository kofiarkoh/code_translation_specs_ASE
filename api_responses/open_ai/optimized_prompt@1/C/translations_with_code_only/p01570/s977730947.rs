// AOJ 2456: Usoperanto
// 2018.4.17 bal4u

use std::io::{self, BufRead};
use std::ptr;

fn in_number() -> i32 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.by_ref().bytes().next().unwrap().unwrap() as char;

    if c == '-' {
        c = handle.by_ref().bytes().next().unwrap().unwrap() as char;
        while c.is_digit(10) {
            n = 10 * n + (c as i32 - '0' as i32);
            c = handle.by_ref().bytes().next().unwrap().unwrap() as char;
        }
        return -n;
    }

    while c.is_digit(10) {
        n = 10 * n + (c as i32 - '0' as i32);
        c = handle.by_ref().bytes().next().unwrap().unwrap() as char;
    }
    n
}

const MAX: usize = 1_000_002;

fn cmp(a: &i64, b: &i64) -> std::cmp::Ordering {
    if a == b {
        std::cmp::Ordering::Equal
    } else if a < b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Less
    }
}

fn calc(u: usize, m: &[i32], hi: &[usize], to: &[Vec<usize>], s: &mut [i64], dp: &mut [i64]) {
    let mut sz = 0;
    s[u] = m[u] as i64;
    dp[u] = 0;
    let mut b = vec![0; hi[u]];

    for &v in &to[u] {
        b[sz] = s[v];
        sz += 1;
        s[u] += s[v];
        dp[u] += dp[v];
    }

    b.sort_by(|a, b| cmp(a, b));
    for i in 0..sz {
        dp[u] += b[i] * i as i64;
    }
}

fn main() {
    let n = in_number() as usize;
    let mut m = vec![0; n];
    let mut memo = vec![0; n];
    let mut hi = vec![0; n];
    let mut base = Vec::new();
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut que = vec![0; MAX];
    let mut a = vec![0; n];
    let mut s = vec![0; n];
    let mut dp = vec![0; n];
    let mut bsz = 0;

    for i in 0..n {
        m[i] = in_number();
        memo[i] = in_number();
        if memo[i] == -1 {
            bsz += 1;
        } else {
            hi[memo[i] as usize] += 1;
        }
    }

    base.resize(bsz);
    bsz = 0;
    for i in 0..n {
        let l = memo[i];
        if l == -1 {
            base[bsz] = i;
            bsz += 1;
        } else {
            to[l as usize].push(i);
        }
    }

    let mut ans = 0;
    for &b in &base {
        let mut top = 1;
        que[1] = b;

        while top > 0 {
            let t = que[top];
            if a[t] < hi[t] {
                que[top + 1] = to[t][a[t]];
                a[t] += 1;
                top += 1;
            } else {
                calc(t, &m, &hi, &to, &mut s, &mut dp);
                top -= 1;
            }
        }
        ans += dp[b];
    }
    println!("{}", ans);
}
