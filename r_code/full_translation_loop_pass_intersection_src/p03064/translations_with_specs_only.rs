use std::io::{self, Read};

const MOD: i64 = 998_244_353;

fn norm_mod(x: i64) -> i64 {
    let mut v = x % MOD;
    if v < 0 {
        v += MOD;
    }
    v
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Behavior is unspecified if the first integer cannot be read.
    // We choose the narrowest reasonable behavior: do nothing (no output).
    let _first = match it.next() {
        Some(s) => s,
        None => return,
    };

    // Read remaining integers (assumed nonnegative per spec).
    let mut a: Vec<usize> = Vec::new();
    let mut s_total: usize = 0;
    for tok in it {
        let v: usize = tok.parse().unwrap();
        s_total = s_total.saturating_add(v);
        a.push(v);
    }

    // Fixed-size arrays in the original; here we size exactly to S+1.
    // Spec assumes S is small enough to avoid out-of-bounds.
    let mut d = vec![0i64; s_total + 1];
    let mut e = vec![0i64; s_total + 1];
    d[0] = 1;
    e[0] = 1;

    let mut running_sum: usize = 0;
    let mut r: i64 = 1; // baseline 3^n mod MOD

    for &ai in &a {
        // Update in decreasing order so each ai is used at most once.
        for t in (0..=running_sum).rev() {
            let to = t + ai;
            // D[to] += 2 * D[t]
            d[to] = (d[to] + 2 * d[t]) % MOD;
            // E[to] += E[t]
            e[to] = (e[to] + e[t]) % MOD;
        }
        running_sum += ai;
        r = (r * 3) % MOD;
    }

    // Finalization:
    // Start from R = 3^n, then subtract 3*D[t] for t <= floor(S/2),
    // and if S even subtract 3*E[S/2].
    let mut ans: i64 = r;

    let half = s_total / 2;
    for t in 0..=half {
        ans -= 3 * d[t];
    }
    if s_total % 2 == 0 {
        ans -= 3 * e[half];
    }

    ans = norm_mod(ans);
    println!("{}", ans);
}
