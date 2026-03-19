use std::io::{self, BufRead};
use std::ptr;

fn in_number() -> i32 {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();
    let mut chars = buffer.chars();
    let first_char = chars.next().unwrap();
    let mut n = 0;

    if first_char == '-' {
        for c in chars {
            if c.is_digit(10) {
                n = n * 10 - (c as i32 - '0' as i32);
            } else {
                break;
            }
        }
        return n;
    } else {
        n = first_char.to_digit(10).unwrap() as i32;
        for c in chars {
            if c.is_digit(10) {
                n = n * 10 + (c as i32 - '0' as i32);
            } else {
                break;
            }
        }
        return n;
    }
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
        s[u] += s[v];
        dp[u] += dp[v];
        sz += 1;
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
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut base = Vec::new();
    let mut bsz = 0;
    let mut a = vec![0; n];
    let mut s = vec![0; n];
    let mut dp = vec![0; n];
    let mut ans = 0;

    for i in 0..n {
        m[i] = in_number();
        memo[i] = in_number();
        if memo[i] == -1 {
            bsz += 1;
        } else {
            hi[memo[i] as usize] += 1;
        }
    }

    base.resize(bsz, 0);
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

    for i in 0..bsz {
        let mut que = vec![0; n];
        let mut top = 0;
        que[top] = base[i];
        top += 1;

        while top > 0 {
            top -= 1;
            let t = que[top];
            if a[t] < hi[t] {
                que[top] = to[t][a[t]];
                a[t] += 1;
                top += 1;
            } else {
                calc(t, &m, &hi, &to, &mut s, &mut dp);
            }
        }
        ans += dp[base[i]];
    }

    println!("{}", ans);
}
