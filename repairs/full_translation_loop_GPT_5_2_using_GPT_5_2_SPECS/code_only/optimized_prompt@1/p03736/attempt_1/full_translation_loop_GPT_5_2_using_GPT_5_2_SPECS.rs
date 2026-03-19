use std::cmp::min;
use std::io::{self, Read};

const N: usize = 200_000;
const INF: i64 = 0x3f3f3f3f3f3f3f3f_i64;

fn update(ft: &mut [i64], mut i: usize, n: usize, x: i64) {
    while i < n {
        ft[i] = min(ft[i], x);
        i |= i + 1;
    }
}

fn query(ft: &[i64], mut i: isize) -> i64 {
    let mut x = INF;
    while i >= 0 {
        let ui = i as usize;
        x = min(x, ft[ui]);
        // C: i &= i + 1, i--;
        i = (i & (i + 1)) - 1;
    }
    x
}

fn abs_i32_to_i64(a: i32) -> i64 {
    // C's abs(int) is undefined for INT_MIN; we emulate typical 2's-complement wrap
    // by doing wrapping negation in i32 and widening.
    if a >= 0 {
        a as i64
    } else {
        a.wrapping_neg() as i64
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let mut q: i32 = it.next().unwrap().parse().unwrap();
    let mut i_cur: i32 = it.next().unwrap().parse().unwrap();
    let j: i32 = it.next().unwrap().parse().unwrap();
    i_cur -= 1;
    let j0 = j - 1;

    // Static arrays in C are zero-initialized; dp starts at 0.
    // ft1/ft2 are memset to 0x3f for n elements.
    let mut dp = vec![0i64; N];
    let mut ft1 = vec![INF; N];
    let mut ft2 = vec![INF; N];

    // Only first n elements are used/initialized in C; we keep updates within n.
    update(&mut ft1[..n], j0 as usize, n, -(j0 as i64));
    update(&mut ft2[..n], (n as i32 - 1 - j0) as usize, n, j0 as i64);

    let mut d: i64 = 0;

    while q > 0 {
        q -= 1;
        let mut i_next: i32 = it.next().unwrap().parse().unwrap();
        i_next -= 1;

        d = d.wrapping_add(abs_i32_to_i64(i_cur.wrapping_sub(i_next)));

        let i_next_usize = i_next as usize;
        let q1 = query(&ft1[..n], i_next as isize).wrapping_add(i_next as i64);
        let q2 = query(&ft2[..n], (n as i32 - 1 - i_next) as isize).wrapping_sub(i_next as i64);
        let base = min(q1, q2);
        let x = base.wrapping_sub(abs_i32_to_i64(i_cur.wrapping_sub(i_next)));

        let i_cur_usize = i_cur as usize;
        dp[i_cur_usize] = min(dp[i_cur_usize], x);

        update(&mut ft1[..n], i_cur_usize, n, x.wrapping_sub(i_cur as i64));
        update(
            &mut ft2[..n],
            (n as i32 - 1 - i_cur) as usize,
            n,
            x.wrapping_add(i_cur as i64),
        );

        i_cur = i_next;
    }

    let mut ans = INF;
    for idx in 0..n {
        ans = min(ans, dp[idx]);
    }
    ans = ans.wrapping_add(d);

    print!("{}\n", ans);
}
