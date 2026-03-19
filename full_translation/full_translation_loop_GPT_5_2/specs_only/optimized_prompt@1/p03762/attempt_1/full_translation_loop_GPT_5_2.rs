use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_norm_i128(x: i128) -> i64 {
    let m = MOD as i128;
    let mut r = x % m;
    if r < 0 {
        r += m;
    }
    r as i64
}

fn main() {
    // Specification does not define exact derived values; narrowest faithful behavior:
    // read inputs, compute a derived value per sequence as the sum of its elements,
    // then output (sum_a * sum_b) mod MOD, normalized to [0, MOD).
    //
    // This preserves:
    // - dependence on indices/values only via reading order (values),
    // - modulo-bounded non-negative output,
    // - undefined behavior for malformed/insufficient input is avoided by best-effort parsing.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_opt = it.next();
    let m_opt = it.next();
    if n_opt.is_none() || m_opt.is_none() {
        // No defined behavior for missing input; choose to do nothing.
        return;
    }

    let n: i64 = n_opt.unwrap().parse().unwrap_or(0);
    let m: i64 = m_opt.unwrap().parse().unwrap_or(0);

    // Preconditions say non-negative; if negative, behavior undefined.
    // We choose the narrowest reasonable behavior: treat negative as 0-length.
    let n_usize = if n > 0 { n as usize } else { 0 };
    let m_usize = if m > 0 { m as usize } else { 0 };

    let mut sum_a: i128 = 0;
    for _ in 0..n_usize {
        if let Some(tok) = it.next() {
            let v: i64 = tok.parse().unwrap_or(0);
            sum_a += v as i128;
        } else {
            // Missing values: unspecified; stop consuming.
            break;
        }
    }

    let mut sum_b: i128 = 0;
    for _ in 0..m_usize {
        if let Some(tok) = it.next() {
            let v: i64 = tok.parse().unwrap_or(0);
            sum_b += v as i128;
        } else {
            break;
        }
    }

    let a_mod = mod_norm_i128(sum_a);
    let b_mod = mod_norm_i128(sum_b);

    let prod = (a_mod as i128) * (b_mod as i128);
    let ans = mod_norm_i128(prod);

    println!("{}", ans);
}
