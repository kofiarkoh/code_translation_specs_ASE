use std::io::{self, BufRead};
use std::cmp::Ordering;

const MAX: usize = 1_000_002;

fn read_input() -> (usize, Vec<i32>, Vec<i32>) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut values = Vec::with_capacity(n);
    let mut parents = Vec::with_capacity(n);
    
    for _ in 0..n {
        values.push(lines.next().unwrap().parse().unwrap());
    }
    
    for _ in 0..n {
        parents.push(lines.next().unwrap().parse().unwrap());
    }
    
    (n, values, parents)
}

fn cmp(a: &i64, b: &i64) -> Ordering {
    b.cmp(a) // Reverse order for sorting
}

fn calc(u: usize, hi: &[usize], to: &[Vec<usize>], m: &[i32], s: &mut [i64], dp: &mut [i64]) {
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

    b.sort_by(cmp);
    for i in 0..sz {
        dp[u] += b[i] * i as i64;
    }
}

fn main() {
    let (n, values, parents) = read_input();
    
    let m = values; // No need for mutability here
    let mut hi = vec![0; n];
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut base = Vec::new();
    
    for i in 0..n {
        let l = parents[i];
        if l == -1 {
            base.push(i);
        } else {
            to[l as usize].push(i);
            hi[l as usize] += 1;
        }
    }

    let mut s = vec![0; n];
    let mut dp = vec![0; n];
    let mut ans = 0;

    for &start in &base {
        let mut stack = vec![start];
        let mut a = vec![0; n];

        while let Some(t) = stack.last() {
            if a[*t] < hi[*t] {
                let next = to[*t][a[*t]];
                stack.push(next);
                a[*t] += 1;
            } else {
                calc(*t, &hi, &to, &m, &mut s, &mut dp);
                stack.pop();
            }
        }
        ans += dp[start];
    }

    println!("{}", ans);
}
