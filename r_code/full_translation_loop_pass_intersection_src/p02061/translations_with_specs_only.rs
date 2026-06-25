use std::io::{self, Read};

const MAX_N: usize = 114_513;

fn main() {
    // Read all input; on any parse failure, exit with nonzero status and no defined output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();

    let q: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let mut queries: Vec<usize> = Vec::with_capacity(q);
    let mut max_n: usize = 0;
    for _ in 0..q {
        let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        if n > max_n {
            max_n = n;
        }
        queries.push(n);
    }

    // Behavior is only defined when max_n <= MAX_N; we do not invent behavior beyond that.
    let limit = max_n.min(MAX_N);

    // Precompute cumulative counts up to `limit`.
    // For each k, we need to decide whether:
    //   prod_{d|k, d<k} d >= 2k
    // The product is treated as capped above by 2k for comparison purposes.
    //
    // We compute the product of proper divisors by multiplying all divisors d < k.
    // To avoid overflow and to respect the "cap", we maintain a capped product:
    //   prod = min(actual_product, 2k)
    // and stop early once prod reaches 2k.
    let mut prefix: Vec<u32> = vec![0; limit + 1];
    for k in 2..=limit {
        let cap: u128 = 2u128 * (k as u128);
        let mut prod: u128 = 1;

        // Enumerate proper divisors via i up to sqrt(k), adding i and k/i when appropriate.
        // Ensure we never include k itself.
        let mut i: usize = 1;
        while i * i <= k {
            if k % i == 0 {
                let d1 = i;
                let d2 = k / i;

                if d1 < k {
                    prod = prod.saturating_mul(d1 as u128);
                    if prod >= cap {
                        prod = cap;
                        break;
                    }
                }
                if d2 != d1 && d2 < k {
                    prod = prod.saturating_mul(d2 as u128);
                    if prod >= cap {
                        prod = cap;
                        break;
                    }
                }
            }
            i += 1;
        }

        let ok = prod >= cap;
        prefix[k] = prefix[k - 1] + if ok { 1 } else { 0 };
    }

    // Answer queries by lookup; for n < 2 output 0.
    // For n > limit (only possible if input exceeds supported bounds), behavior is undefined;
    // we clamp to limit to keep the program total without inventing extra semantics.
    let mut out = String::new();
    for n in queries {
        let ans = if n < 2 {
            0u32
        } else {
            let idx = n.min(limit);
            prefix[idx]
        };
        out.push_str(&ans.to_string());
        out.push('\n');
    }

    print!("{out}");
}
