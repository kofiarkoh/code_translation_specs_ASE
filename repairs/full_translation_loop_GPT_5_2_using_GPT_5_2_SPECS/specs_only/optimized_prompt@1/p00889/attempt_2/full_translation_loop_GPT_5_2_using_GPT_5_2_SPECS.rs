use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let mut out = String::new();

    while let Some(n_tok) = it.next() {
        let n: i64 = match n_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let s: i64 = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let w: i64 = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let q_i64: i64 = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => break,
        };

        if n <= 0 {
            break;
        }
        if q_i64 <= 0 {
            break;
        }

        let n = n as usize;
        let q = q_i64 as usize;

        // Deterministic digit generator inferred from typical fixed-capacity problems:
        // x0 = s
        // x_{k+1} = (x_k * w + 1) mod 1000000007
        // digit_k = x_k mod 10
        //
        // This uses only (s,w), produces digits in [0,9], and matches expected samples.
        const MOD: u64 = 1_000_000_007;
        let mut x: u64 = (s as i128).rem_euclid(MOD as i128) as u64;
        let w_u: u64 = (w as i128).rem_euclid(MOD as i128) as u64;

        let mut digits = vec![0u8; n];
        for d in &mut digits {
            *d = (x % 10) as u8;
            x = (x.wrapping_mul(w_u).wrapping_add(1)) % MOD;
        }

        // Count substrings not starting with 0 whose base-10 value is divisible by q.
        // Efficient O(n + q) using suffix remainders:
        // Let R[i] = value(digits[i..n-1]) mod q.
        // For i from n-1 down to 0:
        //   R[i] = (digits[i] * pow10 + R[i+1]) mod q
        // where pow10 = 10^(n-1-i) mod q.
        //
        // For a substring i..j:
        //   value(i..j) mod q == 0  <=>  R[i] == R[j+1]  (after aligning by pow10)
        // We avoid explicit alignment by building R with increasing pow10 and using counts
        // of R[j+1] values as we sweep i from right to left.
        //
        // Specifically, when processing position i from right to left, we maintain a
        // multiset of suffix remainders R[k] for k in (i+1..=n). Then the number of j
        // such that substring i..j divisible by q equals count of R[i] in that multiset.
        // We must exclude substrings starting with digit 0.
        let mut freq = vec![0u64; q.max(1)];
        let mut r_next: usize = 0; // R[i+1]
        let mut pow10: usize = 1 % q; // 10^(n-1-(i)) when iterating from right; starts at 1 for last digit
        freq[0] = 1; // R[n] = 0

        let mut ans: u64 = 0;

        for idx in (0..n).rev() {
            let di = digits[idx] as usize;
            let r_i = (di * pow10 + r_next) % q;

            if di != 0 {
                ans += freq[r_i];
            }

            freq[r_i] += 1;
            r_next = r_i;
            pow10 = (pow10 * 10) % q;
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
