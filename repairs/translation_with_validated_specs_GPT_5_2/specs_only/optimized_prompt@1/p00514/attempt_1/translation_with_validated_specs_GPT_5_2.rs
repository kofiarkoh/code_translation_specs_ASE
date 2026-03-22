use std::io::{self, Read};

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};

fn main() {
    // Read entire stdin and parse three C `int`-like values.
    // The specification says behavior is unspecified if not exactly three are read;
    // we keep behavior minimal by exiting silently in that case.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };
    let m: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };
    let r: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };

    // C `int` overflow in `n*m` or subtraction is undefined per spec.
    // Rust cannot represent UB; we choose a narrow, deterministic emulation:
    // perform wrapping i32 arithmetic to model typical 2's-complement behavior.
    let r_prime: i32 = r.wrapping_sub(n.wrapping_mul(m));

    if r_prime < 0 {
        println!("0");
        return;
    }
    if r_prime == 0 {
        println!("1");
        return;
    }

    // For meaningful behavior, spec assumes n >= 1 when r' > 0.
    // If not, behavior is unspecified; we avoid inventing extra behavior and just exit.
    if n <= 0 {
        return;
    }

    // Compute C(n + r' - 1, n - 1) using big-integer arithmetic.
    // Use k = min(n-1, r') to reduce work, as described.
    let n_minus_1: u32 = (n - 1) as u32;
    let rp_u32: u32 = r_prime as u32;
    let k: u32 = n_minus_1.min(rp_u32);

    // N = n + r' - 1 (in mathematical integers; spec does not define overflow here,
    // but the branch is only meaningful under its assumptions).
    let n_total: u64 = (n as i64 + r_prime as i64 - 1) as u64;

    // Compute product_{i=1..k} (N - k + i) / i
    // This yields an integer at each step if done with exact division at the end,
    // but we keep it simple: multiply all numerators then divide by all denominators.
    let mut num = BigUint::one();
    let mut den = BigUint::one();

    let start: u64 = n_total - k as u64 + 1;
    for i in 0..k as u64 {
        num *= BigUint::from(start + i);
    }
    for i in 1..=k as u64 {
        den *= BigUint::from(i);
    }

    // Exact division is expected under the stated combinatorial formula.
    let result = num / den;
    println!("{result}");
}
