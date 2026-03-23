use std::cmp::min;
use std::io::{self, Read};

const N: usize = 200_000;
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
        let ui = i as usize;
        x = min(x, ft[ui]);
        // C: i &= i + 1, i--;
        i = (i & (i + 1)) - 1;
    }
    x
}

fn abs_i32_to_i64(v: i32) -> i64 {
    // C's abs(int) is undefined for INT_MIN; we emulate typical 2's complement wrap
    // by doing wrapping negation in i32 and widening.
    if v >= 0 {
        v as i64
    } else {
        v.wrapping_neg() as i64
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let mut q: i32 = it.next().unwrap().parse().unwrap();
    let mut i_cur: i32 = it.next().unwrap().parse().unwrap();
    let j_in: i32 = it.next().unwrap().parse().unwrap();
    i_cur -= 1;
    let j0: i32 = j_in - 1;

    // Static arrays in C; here fixed-size vectors.
    let mut dp = vec![INF; N];
    let mut ft1 = vec![INF; N];
    let mut ft2 = vec![INF; N];

    // Only first n elements are used/initialized in C via memset(n * sizeof).
    // We already filled all with INF; that's fine.
    update(&mut ft1[..n], j0 as usize, n, -(j0 as i64));
    update(&mut ft2[..n], (n as i32 - 1 - j0) as usize, n, j0 as i64);

    let mut d: i64 = 0;

    while q > 0 {
        q -= 1;
        let mut i_next: i32 = it.next().unwrap().parse().unwrap();
        i_next -= 1;

        d = d.wrapping_add(abs_i32_to_i64(i_cur.wrapping_sub(i_next)));

        let qi1 = query(&ft1[..n], i_next as isize).wrapping_add(i_next as i64);
        let qi2 = query(&ft2[..n], (n as i32 - 1 - i_next) as isize).wrapping_sub(i_next as i64);
        let base = min(qi1, qi2);
        let x = base.wrapping_sub(abs_i32_to_i64(i_cur.wrapping_sub(i_next)));

        let idx = i_cur as usize;
        dp[idx] = min(dp[idx], x);

        update(&mut ft1[..n], idx, n, x.wrapping_sub(i_cur as i64));
        update(
            &mut ft2[..n],
            (n - 1 - idx),
            n,
            x.wrapping_add(i_cur as i64),
        );

        i_cur = i_next;
    }

    let mut ans = INF;
    for k in 0..n {
        ans = min(ans, dp[k]);
    }
    ans = ans.wrapping_add(d);

    print!("{}\n", ans);
}
