use std::io::{self, Read};

/// Computes the transform f(s) = (s*s / 100) mod 10000.
///
/// The C program uses `int` for intermediate multiplication. The SPEC
/// requires that inputs are such that s*s fits into the signed int
/// range; here we perform the multiplication in i64 to avoid accidental
/// Rust debug-overflow panics while preserving the intended mathematical
/// result when the precondition holds.
fn solve(s: i32) -> i32 {
    let ss = s as i64;
    let n = ss * ss / 100; // integer division, same semantics for non-negative values
    // ensure result is in 0..9999
    let mut r = (n % 10000) as i32;
    if r < 0 {
        // defensive: if somehow negative, normalize (though with valid inputs n >= 0)
        r += 10000;
    }
    r
}

fn main() {
    // Read all input from stdin and split by whitespace, similar to repeated scanf("%d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut toks = input.split_whitespace();

    // Read d. Per SPEC, d is a signed int and must be >= 0.
    let d: i32 = match toks.next() {
        Some(s) => s.parse().expect("failed to parse d"),
        None => return, // unspecified in spec if missing; exit quietly
    };

    if d < 0 {
        // SPEC requires d >= 0. If violated, behavior is unspecified; choose to do nothing.
        return;
    }

    // Process each test case
    for case_idx in 1..=d as usize {
        // Read seed s for this test case
        let s0: i32 = match toks.next() {
            Some(s) => s.parse().expect("failed to parse seed"),
            None => return, // unspecified in spec; exit
        };

        println!("Case {}:", case_idx);
        let mut s = s0;
        for _ in 0..10 {
            s = solve(s);
            println!("{}", s);
        }
        // After the loop, the in-memory `s` equals the tenth iterate (as per SPEC).
    }
}
