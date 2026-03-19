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
    // Use i128 to avoid overflow; spec assumes no overflow in original, but Rust i64 can overflow.
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
    // MOD is prime.
    mod_pow(a, MOD - 2)
}

fn digits_len(mut x: i64) -> i64 {
    // x is assumed positive by spec.
    let mut d = 0i64;
    while x > 0 {
        d += 1;
        x /= 10;
    }
    d
}

fn main() {
    // Read one integer from stdin.
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

    // Interpret the described computation as:
    // Sum over k=1..n of (k * digits_len(k)) modulo MOD.
    //
    // This matches the "count-like quantity" tied to decimal digit-length constraints,
    // and uses fixed bounds for lengths 1..8, with additional contributions for lengths > 8.
    //
    // Compute efficiently by grouping ranges of equal digit length.
    let inv2 = mod_inv(2);
    let mut ans = 0i64;

    // Handle lengths 1..8 using the fixed bounds table.
    let mut start: i64 = 1;
    for len in 1..=8 {
        if start > n {
            break;
        }
        let end = std::cmp::min(n, start + bounds[len] - 1);
        // sum_{k=start..end} k = (start+end)*(end-start+1)/2
        let cnt = end - start + 1;
        let sum = mod_mul(mod_mul(mod_norm(start + end), mod_norm(cnt)), inv2);
        ans = mod_add(ans, mod_mul(sum, len as i64));
        start = end + 1;
    }

    // Lengths > 8: continue by powers of 10.
    // start is already the first number not covered above.
    let mut len = 9i64;
    let mut next_pow10: i64 = 1;
    for _ in 0..8 {
        next_pow10 *= 10;
    }
    // next_pow10 is 10^8; for len=9, range starts at 10^8.
    // Ensure start is at least 10^8.
    if start < next_pow10 {
        start = next_pow10;
    }
    while start <= n {
        // end is min(n, 10^len - 1). Compute 10^len carefully in i128.
        let pow10_len = {
            let mut v: i128 = 1;
            for _ in 0..len {
                v *= 10;
            }
            v
        };
        let end_i128 = std::cmp::min(n as i128, pow10_len - 1);
        let end = end_i128 as i64;

        let cnt = end - start + 1;
        let sum = mod_mul(mod_mul(mod_norm(start + end), mod_norm(cnt)), inv2);
        ans = mod_add(ans, mod_mul(sum, len));

        start = end + 1;
        len += 1;
    }

    println!("{}", mod_norm(ans));
}
