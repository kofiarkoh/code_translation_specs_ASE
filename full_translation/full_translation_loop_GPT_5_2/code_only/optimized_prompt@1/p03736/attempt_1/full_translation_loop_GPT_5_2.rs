use std::cmp::min;
use std::io::{self, Read};

const N: usize = 200000;
const INF: i64 = 0x3f3f3f3f3f3f3f3f;

fn update(ft: &mut [i64], mut i: usize, n: usize, x: i64) {
    while i < n {
        ft[i] = min(ft[i], x);
        i |= i + 1;
    }
}

fn query(ft: &[i64], mut i: isize) -> i64 {
    let mut x = INF;
    while i >= 0 {
        let iu = i as usize;
        x = min(x, ft[iu]);
        // C: i &= i + 1, i--;
        i = ((i as usize) & ((i as usize) + 1)) as isize - 1;
    }
    x
}

fn abs_i64(x: i64) -> i64 {
    // C's abs() is for int; here we need i64 absolute value.
    // Inputs are within reasonable bounds (indices), so no i64::MIN issue.
    if x < 0 { -x } else { x }
}

fn main() {
    // Read all input (scanf-like whitespace parsing)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut q: i32 = it.next().unwrap().parse().unwrap();
    let mut i_cur: i64 = it.next().unwrap().parse::<i64>().unwrap() - 1;
    let j: i64 = it.next().unwrap().parse::<i64>().unwrap() - 1;

    // Static arrays in C are zero-initialized; dp starts at 0.
    let mut dp = vec![0i64; N];
    let mut ft1 = vec![INF; N];
    let mut ft2 = vec![INF; N];

    // Only first n elements are relevant; C memset uses n * sizeof.
    // We already initialized whole arrays to INF, which is equivalent.

    update(&mut ft1, j as usize, n, -j);
    update(&mut ft2, (n as i64 - 1 - j) as usize, n, j);

    let mut d: i64 = 0;

    while q > 0 {
        q -= 1;
        let i_next: i64 = it.next().unwrap().parse::<i64>().unwrap() - 1;

        d += abs_i64(i_cur - i_next);

        let q1 = query(&ft1, i_next as isize).wrapping_add(i_next);
        let q2 = query(&ft2, (n as i64 - 1 - i_next) as isize).wrapping_sub(i_next);
        let mut x = min(q1, q2);
        x = x.wrapping_sub(abs_i64(i_cur - i_next));

        // dp[i] = min(dp[i], x); dp initially 0 as in C static storage.
        let idx = i_cur as usize;
        dp[idx] = min(dp[idx], x);

        update(&mut ft1, idx, n, x.wrapping_sub(i_cur));
        update(&mut ft2, (n as i64 - 1 - i_cur) as usize, n, x.wrapping_add(i_cur));

        i_cur = i_next;
    }

    let mut ans = INF;
    for k in 0..n {
        ans = min(ans, dp[k]);
    }
    ans = ans.wrapping_add(d);

    print!("{ans}\n");
}
