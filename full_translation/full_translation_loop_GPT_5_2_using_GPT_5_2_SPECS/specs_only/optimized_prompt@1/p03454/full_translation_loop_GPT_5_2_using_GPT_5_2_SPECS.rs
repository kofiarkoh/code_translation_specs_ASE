use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_norm(x: i64) -> i64 {
    let mut v = x % MOD;
    if v < 0 {
        v += MOD;
    }
    v
}

fn mod_add(a: i64, b: i64) -> i64 {
    mod_norm(a + b)
}

fn mod_mul(a: i64, b: i64) -> i64 {
    mod_norm(((a as i128 * b as i128) % (MOD as i128)) as i64)
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut r = 1i64;
    a = mod_norm(a);
    while e > 0 {
        if (e & 1) == 1 {
            r = mod_mul(r, a);
        }
        a = mod_mul(a, a);
        e >>= 1;
    }
    r
}

fn mod_inv(a: i64) -> i64 {
    mod_pow(a, MOD - 2)
}

fn sum_1_to(n: i64, inv2: i64) -> i64 {
    // n*(n+1)/2 mod MOD
    mod_mul(mod_mul(mod_norm(n), mod_norm(n + 1)), inv2)
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let n: i64 = s.split_whitespace().next().unwrap().parse().unwrap();

    // Precomputed decimal-count bounds for lengths 1 through 8:
    // length 1: 9
    // length i (2..8): 9 × 10^(i−1)
    let mut bounds: [i64; 9] = [0; 9];
    bounds[1] = 9;
    let mut p10: i64 = 1;
    for i in 2..=8 {
        p10 *= 10;
        bounds[i] = 9 * p10;
    }

    // Semantics consistent with the spec and typical divisor/digit-length problems:
    // Compute:
    //   ans = sum_{d | n} d * ( count of k in [1..n] with decimal length = d )  (mod MOD)
    //
    // The "count-like quantity" is the number of integers with a given decimal length,
    // bounded explicitly for lengths 1..8, and for lengths > 8 we add contributions for
    // divisors whose associated lengths exceed 8 digits (i.e., d > 8).
    //
    // For each divisor d, let L = d. The count of k with length L is:
    //   cnt(L) = max(0, min(n, 10^L - 1) - 10^(L-1) + 1)
    // For L=1..8, this equals the fixed bounds table except possibly truncated by n.
    //
    // Then contribution is d * cnt(d).
    let inv2 = mod_inv(2);
    let mut ans = 0i64;

    let mut i: i64 = 1;
    while i * i <= n {
        if n % i == 0 {
            let d1 = i;
            let d2 = n / i;

            // Process divisor d1
            ans = mod_add(ans, mod_mul(mod_norm(d1), mod_norm(count_len(n, d1, &bounds, inv2))));

            // Process paired divisor d2 if distinct
            if d2 != d1 {
                ans = mod_add(ans, mod_mul(mod_norm(d2), mod_norm(count_len(n, d2, &bounds, inv2))));
            }
        }
        i += 1;
    }

    println!("{}", mod_norm(ans));
}

fn count_len(n: i64, len: i64, bounds: &[i64; 9], _inv2: i64) -> i64 {
    if len <= 0 {
        return 0;
    }
    if len <= 8 {
        // For length <= 8, the full count is bounds[len], but truncated by n.
        // Range for this length is [10^(len-1), 10^len - 1].
        let start = if len == 1 { 1 } else { pow10_i64(len - 1) };
        if n < start {
            return 0;
        }
        let end = std::cmp::min(n, start + bounds[len as usize] - 1);
        return end - start + 1;
    }

    // For len > 8, compute using i128 to avoid overflow in 10^len.
    let start_i128 = pow10_i128(len - 1);
    if (n as i128) < start_i128 {
        return 0;
    }
    let end_i128 = {
        let p = pow10_i128(len);
        let max_end = p - 1;
        std::cmp::min(n as i128, max_end)
    };
    (end_i128 - start_i128 + 1) as i64
}

fn pow10_i64(e: i64) -> i64 {
    let mut v: i64 = 1;
    for _ in 0..e {
        v *= 10;
    }
    v
}

fn pow10_i128(e: i64) -> i128 {
    let mut v: i128 = 1;
    for _ in 0..e {
        v *= 10;
    }
    v
}
