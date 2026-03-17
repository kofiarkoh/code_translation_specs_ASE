use std::io::{self, BufRead};
use std::cmp;

const N: usize = 200000;
const INF: i64 = 0x3f3f3f3f3f3f3f3f;

fn min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

fn update(ft: &mut [i64], i: usize, n: usize, x: i64) {
    let mut idx = i;
    while idx < n {
        ft[idx] = min(ft[idx], x);
        idx |= idx + 1;
    }
}

fn query(ft: &[i64], i: usize) -> i64 {
    let mut x = INF;
    let mut idx = i;
    while idx != usize::MAX {
        x = min(x, ft[idx]);
        idx &= idx + 1;
        if idx == 0 { break; }
        idx -= 1;
    }
    x
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    let mut i: usize = parts.next().unwrap().parse().unwrap() - 1;
    let mut j: usize = parts.next().unwrap().parse().unwrap() - 1;

    let mut dp = vec![INF; N];
    let mut ft1 = vec![INF; N];
    let mut ft2 = vec![INF; N];

    update(&mut ft1, j, n, -(j as i64));
    update(&mut ft2, n - 1 - j, n, j as i64);
    let mut d = 0;

    for _ in 0..q {
        let i_: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap() - 1;
        d += (i as isize - i_ as isize).abs() as i64;

        let x = min(query(&ft1, i_) + i_ as i64, query(&ft2, n - 1 - i_) - i_ as i64) - (i as isize - i_ as isize).abs() as i64;
        dp[i] = min(dp[i], x);
        update(&mut ft1, i, n, x - i as i64);
        update(&mut ft2, n - 1 - i, n, x + i as i64);
        i = i_;
    }

    let mut ans = INF;
    for k in 0..n {
        ans = min(ans, dp[k]);
    }
    ans += d;

    println!("{}", ans);
}
