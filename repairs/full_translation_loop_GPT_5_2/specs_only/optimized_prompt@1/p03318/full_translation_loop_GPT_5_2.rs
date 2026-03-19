use std::io::{self, Read};

fn digit_sum(mut x: u64) -> u64 {
    let mut s = 0u64;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn main() {
    // Read a single non-negative integer: number of iterations.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = match input.split_whitespace().next() {
        Some(tok) => tok.parse::<usize>().unwrap(),
        None => return,
    };

    // Invariant: current value is always > 0.
    let mut cur: u64 = 1;

    // For each iteration, output current value, then update it to a subsequent value
    // that minimizes the ratio x / sum_digits(x) among candidates derived from cur.
    //
    // The specification does not define the candidate set precisely; the narrowest
    // reasonable interpretation consistent with "subsequent value" and typical digit-sum
    // ratio minimization is to consider numbers formed by taking a prefix of `cur` and
    // replacing the remaining suffix with all 9s, then choosing the best among those
    // and `cur` itself, and finally advancing to the next integer after the chosen one.
    //
    // This yields a well-defined, deterministic sequence and preserves the invariant.
    let mut out = String::new();
    for _ in 0..n {
        out.push_str(&format!("{cur}\n"));

        // Build candidates: cur itself, and for each position, set suffix digits to 9.
        let s = cur.to_string();
        let len = s.len();

        let mut best = cur;
        let mut best_num = cur;
        let mut best_den = digit_sum(cur).max(1); // cur>0, but keep denominator non-zero.

        for i in 0..len {
            // Candidate: keep first i digits, then fill the rest with '9'.
            // i == 0 => all 9s of same length.
            let mut cand_str = String::with_capacity(len);
            cand_str.push_str(&s[..i]);
            for _ in i..len {
                cand_str.push('9');
            }
            let cand = cand_str.parse::<u64>().unwrap();

            if cand >= cur {
                let den = digit_sum(cand).max(1);
                // Compare cand/den < best_num/best_den using cross-multiplication.
                // Use u128 to avoid overflow in multiplication.
                let lhs = (cand as u128) * (best_den as u128);
                let rhs = (best_num as u128) * (den as u128);
                if lhs < rhs || (lhs == rhs && cand < best) {
                    best = cand;
                    best_num = cand;
                    best_den = den;
                }
            }
        }

        // Transition: move to a subsequent value.
        cur = best.saturating_add(1);
        if cur == 0 {
            // Saturation could only produce 0 if best was u64::MAX; keep invariant by forcing 1.
            cur = 1;
        }
    }

    print!("{out}");
}
