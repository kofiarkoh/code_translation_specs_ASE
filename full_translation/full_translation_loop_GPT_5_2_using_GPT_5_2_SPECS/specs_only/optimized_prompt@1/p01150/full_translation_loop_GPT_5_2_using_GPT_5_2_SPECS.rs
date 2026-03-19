use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    // 8 distinct individuals => multiply by 8! at the end.
    let fact8: u128 = 40320;

    let mut out = String::new();

    while let Some(tok) = it.next() {
        let n: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // Unspecified for malformed input; stop.
        };
        if n == 0 {
            break;
        }

        if n <= 16 {
            out.push_str("0\n");
            continue;
        }

        // Count ways to choose 8 seats on a circle of size n with no two adjacent (including wrap).
        //
        // Use the standard cyclic non-adjacent k-subset count:
        //   ways_choose = n/(n-k) * C(n-k, k)
        // This is an integer for valid parameters.
        //
        // Then multiply by 8! for distinct individuals.
        let n_u = n as u128;
        let k_u: u128 = 8;

        let comb = binom_u128(n_u - k_u, k_u);
        let ways_choose = mul_div_exact_u128(n_u, comb, n_u - k_u);

        let ans = ways_choose.wrapping_mul(fact8);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}

fn mul_div_exact_u128(a: u128, b: u128, d: u128) -> u128 {
    // Compute (a*b)/d assuming exact division in mathematical intent.
    // Reduce by gcds to avoid overflow; still uses wrapping mul as overflow behavior is unspecified.
    if d == 0 {
        return 0; // Not expected for valid inputs (n>k), but keep narrow and total.
    }
    let g1 = gcd_u128(a, d);
    let a1 = a / g1;
    let d1 = d / g1;

    let g2 = gcd_u128(b, d1);
    let b1 = b / g2;
    let d2 = d1 / g2;

    // Now d2 should be 1 for exact division.
    let prod = a1.wrapping_mul(b1);
    if d2 == 1 {
        prod
    } else {
        // If not fully reduced (shouldn't happen for intended domain), do integer division.
        prod / d2
    }
}

fn binom_u128(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    let k = std::cmp::min(k, n - k);
    if k == 0 {
        return 1;
    }

    // Multiplicative with gcd reduction.
    let mut numer: Vec<u128> = (0..k).map(|i| n - k + 1 + i).collect();
    let mut denom: Vec<u128> = (1..=k).collect();

    for d in denom.iter_mut() {
        let mut x = *d;
        if x == 1 {
            continue;
        }
        for a in numer.iter_mut() {
            if x == 1 {
                break;
            }
            let g = gcd_u128(*a, x);
            if g > 1 {
                *a /= g;
                x /= g;
            }
        }
        *d = x;
    }

    let mut res: u128 = 1;
    for a in numer {
        res = res.wrapping_mul(a);
    }
    res
}

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
