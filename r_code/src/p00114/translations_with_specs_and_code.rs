// lcm_orders.rs
//
// Translated from the provided C program and guided by the specification.
// Reads groups of six int64 values (a1 m1 a2 m2 a3 m3) from stdin until
// a group of six zeros is encountered. For each group, computes the
// multiplicative order ci (smallest positive k with a_i^k ≡ 1 (mod m_i))
// for i=1..3 using an iterative approach identical to the C code, then
// prints the LCM of the three orders as a signed 64-bit integer.
//
// The translation follows the SPEC as authoritative. The implementation
// aims to be safe and idiomatic Rust, preserving semantics. No `unsafe`
// is used.

use std::io::{self, Read};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    // Matches the Euclidean algorithm used in the C code.
    // Precondition (from SPEC): a and b will be positive here.
    while a != 0 {
        let t = a;
        a = b % a;
        b = t;
    }
    b
}

fn lcm(a: i64, b: i64) -> i64 {
    // Compute lcm = (a / gcd(a,b)) * b to reduce overflow risk.
    // This is semantically equivalent to a*b/gcd(a,b) under the SPEC
    // precondition that overflow does not occur.
    let g = gcd(a, b);
    (a / g) * b
}

fn main() {
    // Read all stdin, split on whitespace, parse as i64 tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut toks = input.split_whitespace();
    loop {
        // Attempt to read six integers. If fewer than six tokens remain,
        // stop (the SPEC leaves behavior on malformed/short input unspecified;
        // this is a safe and reasonable behavior).
        let a1 = match toks.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let m1 = match toks.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let a2 = match toks.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let m2 = match toks.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let a3 = match toks.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => break,
        };
        let m3 = match toks.next().and_then(|s| s.parse::<i64>().ok()) {
            Some(v) => v,
            None => break,
        };

        // Termination sentinel: all six zeros.
        if a1 == 0 && m1 == 0 && a2 == 0 && m2 == 0 && a3 == 0 && m3 == 0 {
            break;
        }

        // Compute multiplicative order ci for each pair (ai, mi)
        // Semantic fidelity: follow the exact iterative scheme from C:
        // c = 1; r = a; while r != 1 { r = r * a % m; ++c; }
        // Note: Precondition (SPEC) ensures mi > 0 and that such c exists.
        let c1 = {
            let mut c: i64 = 1;
            let mut r: i64 = a1;
            // The loop mirrors the C code. If mi <= 0 or no such c exists,
            // behavior may be undefined or non-terminating as per SPEC.
            while r != 1 {
                // r * a1 % m1 -- rely on i64 arithmetic. Precondition prevents overflow.
                r = (r.wrapping_mul(a1)) % m1;
                c += 1;
            }
            c
        };

        let c2 = {
            let mut c: i64 = 1;
            let mut r: i64 = a2;
            while r != 1 {
                r = (r.wrapping_mul(a2)) % m2;
                c += 1;
            }
            c
        };

        let c3 = {
            let mut c: i64 = 1;
            let mut r: i64 = a3;
            while r != 1 {
                r = (r.wrapping_mul(a3)) % m3;
                c += 1;
            }
            c
        };

        let result = lcm(lcm(c1, c2), c3);
        // Print as a signed 64-bit decimal with newline, one per case.
        println!("{}", result);
    }
}
