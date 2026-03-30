use std::cmp::min;
use std::io::{self, BufRead};

const N: usize = 200_000;
const INF: i64 = 0x3f3f3f3f3f3f3f3f;

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
    let mut params = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = params.next().unwrap() as usize;
    let q = params.next().unwrap() as usize;
    let mut i = params.next().unwrap() as usize - 1; // Convert to zero-based index
    let mut j = params.next().unwrap() as usize - 1; // Convert to zero-based index

    let mut dp = vec![INF; n];
    let mut ft1 = vec![INF; n];
    let mut ft2 = vec![INF; n];

    update(&mut ft1, j, n, -(j as i64));
    update(&mut ft2, n - 1 - j, n, j as i64);

    let mut d = 0;

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let i_ = query_line.trim().parse::<usize>().unwrap() - 1; // Convert to zero-based index

        d += (i as i64 - i_ as i64).abs();
        let x = min(query(&ft1, i_), query(&ft2, n - 1 - i_)) - (i as i64 - i_ as i64).abs();
        dp[i] = min(dp[i], x);
        update(&mut ft1, i, n, x - (i as i64));
        update(&mut ft2, n - 1 - i, n, x + (i as i64));
        i = i_;
    }

    let ans = dp.iter().copied().min().unwrap_or(INF) + d;
    println!("{}", ans);
}
