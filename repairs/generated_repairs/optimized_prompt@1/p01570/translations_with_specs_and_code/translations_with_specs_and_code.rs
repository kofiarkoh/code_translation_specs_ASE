use std::io::{self, BufRead};
use std::cmp::Ordering;

const MAX: usize = 1_000_002;

fn in_value() -> i32 {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    trimmed.parse().unwrap_or(0) // Assuming valid input as per spec
}

fn cmp(a: &i64, b: &i64) -> Ordering {
    if a == b {
        Ordering::Equal
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn calc(u: usize, m: &[i32], hi: &[usize], to: &[Vec<usize>], s: &mut [i64], dp: &mut [i64]) {
    let mut sz = 0;
    s[u] = m[u] as i64;
    dp[u] = 0;
    let mut b = vec![0; hi[u]];

    for &v in &to[u] {
        b[sz] = s[v];
        s[u] += s[v];
        dp[u] += dp[v];
        sz += 1;
    }

    b[..sz].sort_by(|a, b| cmp(a, b));
    for i in 0..sz {
        dp[u] += b[i] * (sz as i64 - i as i64 - 1);
    }
}

fn main() {
    let n = in_value() as usize;
    let mut m = vec![0; n];
    let mut memo = vec![0; n];
    let mut hi = vec![0; n];
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut base = Vec::new();
    let mut s = vec![0; n];
    let mut dp = vec![0; n];
    let mut ans = 0;

    for i in 0..n {
        m[i] = in_value();
        memo[i] = in_value();
        if memo[i] == -1 {
            base.push(i);
        } else {
            hi[memo[i] as usize] += 1;
        }
    }

    for i in 0..n {
        if hi[i] > 0 {
            to[i] = vec![0; hi[i]];
        }
    }

    let mut bsz = 0;
    for i in 0..n {
        let l = memo[i];
        if l == -1 {
            bsz += 1;
        } else {
            to[l as usize][hi[l as usize] - 1] = i;
            hi[l as usize] -= 1;
        }
    }

    for &root in &base {
        let mut stack = vec![root];
        let mut a = vec![0; n];

        while let Some(t) = stack.pop() {
            if a[t] < hi[t] {
                stack.push(to[t][a[t]]);
                a[t] += 1;
            } else {
                calc(t, &m, &hi, &to, &mut s, &mut dp);
            }
        }
        ans += dp[root];
    }

    println!("{}", ans);
}
