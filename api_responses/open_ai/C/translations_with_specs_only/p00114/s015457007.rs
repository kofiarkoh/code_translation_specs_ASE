// lcm_orders.rs
//
// Translates the given specification into Rust.
// Reads groups of six signed 64-bit integers (a1 m1 a2 m2 a3 m3) from stdin,
// stops when all six are zero, and for each group prints the LCM of the
// multiplicative orders c_i where a_i^{c_i} ≡ 1 (mod m_i).
//
// The implementation follows the specification closely:
// - Inputs interpreted as i64.
// - Modular sequence produced by computing successive (current * a) % m using
//   signed arithmetic semantics (we perform intermediate multiplications in i128
//   to avoid Rust overflow while preserving sign behavior of remainder).
// - The check for "≡ 1 (mod m)" is implemented as `residue == (1 % m)` (in i128),
//   which matches congruence semantics for any positive m (including m == 1).
// - LCM is computed via gcd and arithmetic is performed in i128 and finally
//   cast to i64 for printing; this preserves correctness when the result fits
//   in signed 64-bit as required by the specification.

use std::io::{self, Read};

fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn multiplicative_order(a: i64, m: i64) -> i128 {
    // Precondition (per specification): m > 0 and an order exists.
    // Work in i128 for intermediate multiplication to avoid overflow,
    // but use signed remainder semantics consistent with C/Rust (% keeps sign of numerator).
    let m_i = m as i128;
    let a_i = a as i128;

    // one_mod = 1 % m (i128). This expresses the target residue for congruence.
    let one_mod = 1_i128 % m_i;

    // curr = a % m (i128). This corresponds to a^1 mod m.
    let mut curr = a_i % m_i;

    // Start counting exponent k = 1
    let mut k: i128 = 1;
    if curr == one_mod {
        return k;
    }

    // Iterate until curr == one_mod (per specification, such k exists).
    loop {
        // curr = (curr * a) % m  (i128 arithmetic)
        curr = (curr * a_i) % m_i;
        k += 1;
        if curr == one_mod {
            return k;
        }
        // No explicit guard for infinite loops: specification states order exists.
    }
}

fn lcm_i128(a: i128, b: i128) -> i128 {
    if a == 0 || b == 0 {
        return 0;
    }
    let g = gcd_i128(a, b);
    // Compute (a / g) * b in i128 to avoid intermediate overflow in Rust.
    (a / g) * b
}

fn main() {
    // Read all stdin and split into tokens of integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Per specification, behavior on I/O failure is unspecified;
        // we simply exit here.
        return;
    }

    let mut nums = Vec::new();
    for tok in input.split_whitespace() {
        // parse as i64; if parse fails, we stop processing further tokens
        // (keeps behavior narrow as per specification).
        match tok.parse::<i64>() {
            Ok(v) => nums.push(v),
            Err(_) => break,
        }
    }

    // Process in chunks of 6. If incomplete final chunk, we stop (spec is unspecified).
    let mut idx = 0usize;
    while idx + 5 < nums.len() {
        let a1 = nums[idx];
        let m1 = nums[idx + 1];
        let a2 = nums[idx + 2];
        let m2 = nums[idx + 3];
        let a3 = nums[idx + 4];
        let m3 = nums[idx + 5];
        idx += 6;

        // Termination sentinel: all six zeros
        if a1 == 0 && m1 == 0 && a2 == 0 && m2 == 0 && a3 == 0 && m3 == 0 {
            // Normal termination
            return;
        }

        // Per specification, we assume m_i > 0 and order exists; no explicit checks.
        // Compute orders as i128
        let c1 = multiplicative_order(a1, m1);
        let c2 = multiplicative_order(a2, m2);
        let c3 = multiplicative_order(a3, m3);

        // Compute LCM(c1, c2, c3)
        let l12 = lcm_i128(c1, c2);
        let l123 = lcm_i128(l12, c3);

        // The spec requires printing as signed 64-bit decimal ("%lld").
        // We cast to i64. The specification guarantees the LCM fits in signed 64-bit.
        let out = l123 as i64;
        println!("{}", out);
    }

    // If we exit the loop without seeing the sentinel, simply return (unspecified behavior covered).
}
