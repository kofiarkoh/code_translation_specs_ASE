use std::io::{self, BufRead};

const N: usize = 200_000;
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
        if idx == 0 {
            break;
        }
        idx &= idx + 1;
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
    let mut i: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;
    let mut j: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1;

    let mut dp = vec![INF; n];
    let mut ft1 = vec![INF; n];
    let mut ft2 = vec![INF; n];

    update(&mut ft1, j, n, -(j as i64));
    update(&mut ft2, n - 1 - j, n, j as i64);

    let mut d = 0;

    for _ in 0..q {
        let i_: usize = lines.next().unwrap().unwrap().parse::<usize>().unwrap() - 1;
        d += (i as isize - i_ as isize).abs() as i64;

        let x = min(query(&ft1, i_) + (i_ as i64), query(&ft2, n - 1 - i_) - (i_ as i64)) - (i as i64 - i_ as i64);
        dp[i_] = min(dp[i_], x); // Update dp at i_ instead of i
        update(&mut ft1, i_, n, x - (i_ as i64)); // Update ft1 at i_
        update(&mut ft2, n - 1 - i_, n, x + (i_ as i64)); // Update ft2 at n - 1 - i_
        i = i_;
    }

    let mut ans = INF;
    for value in dp.iter() {
        ans = min(ans, *value);
    }
    ans += d;

    println!("{}", ans);
}
