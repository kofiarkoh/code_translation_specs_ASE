// gcd_lcm.rs
//
// Translation of the provided C program to Rust, guided by the SPEC.
// Notes about specification vs. C source:
// - SPEC describes the program as repeatedly reading pairs of integers
//   and processing only when two integers are successfully read.
// - The original C code used `while (scanf("%d%d",&a,&b) != EOF)` which
//   continues when scanf returns a positive value other than EOF (e.g. 1).
//   That is inconsistent with the SPEC's precondition. This Rust
//   implementation follows the SPEC: it processes only complete pairs of
//   integers (ignores a trailing single token) and stops on parse failure.
// - To mirror C's potential signed overflow behavior in printed results,
//   multiplications are performed with wrapping arithmetic (i32::wrapping_mul).
//   Division and remainder use Rust's normal operators. This choice is to
//   avoid panics on overflow in debug builds and to provide deterministic
//   two's-complement wrapping behavior similar to typical machine behavior.
//
// Behaviour preserved:
// - Reads whitespace-separated integer tokens from stdin, processes tokens in
//   pairs (a, b). For each pair computes c via the same factor-extraction
//   algorithm and prints "c v\n" where v = tmp_a * tmp_b * c (using wrapping mul).
//
// Limitations / assumptions:
// - Parsing errors stop processing further input (spec considers malformed input
//   unspecified; this approach prevents panics).
// - Uses i32 to model C 'int'.
// - No unsafe code used.

use std::io::{self, Read};

fn main() {
    // Read all of stdin into a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit silently (mirrors C program having no error messages).
        return;
    }

    let mut toks = input.split_whitespace();

    // Process tokens in pairs. If there's an odd trailing token, it is ignored
    // (matches SPEC's assumption that scanf should return two or EOF).
    while let Some(a_str) = toks.next() {
        let b_str = match toks.next() {
            Some(s) => s,
            None => break, // trailing single token: stop processing
        };

        // Parse tokens as i32. On parse error, stop processing further input.
        let mut a: i32 = match a_str.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let mut b: i32 = match b_str.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        let mut c: i32 = 1;
        // Match C code behavior: if b > a, swap so a >= b
        if b > a {
            let tmp = a;
            a = b;
            b = tmp;
        }

        let mut tmp_a = a;
        let mut tmp_b = b;

        // The C code used: for (int i=2; i<=tmp_b; i++) { ... }
        // Because tmp_b is mutated in the loop condition in C, we reproduce that
        // by using a while loop where the condition is checked each iteration.
        let mut i: i32 = 2;
        // If tmp_b is negative or less than 2, the loop will not run (matches C).
        while i <= tmp_b {
            // Perform the inner while(tmp_b % i == 0 && tmp_a % i == 0)
            // Note: modulo with negative numbers behaves consistently with Rust's `%`.
            while tmp_b % i == 0 && tmp_a % i == 0 {
                // Use wrapping multiplication to avoid panics on overflow and to
                // approximate typical two's-complement wraparound behavior in C.
                c = c.wrapping_mul(i);
                tmp_b = tmp_b / i;
                tmp_a = tmp_a / i;
            }
            i = i.wrapping_add(1);
            // wrapping_add is used just to avoid panic on overflow of i (practically unreachable here).
        }

        // Compute v = tmp_a * tmp_b * c as in the C program.
        // Use wrapping_mul to avoid Rust panics on overflow and to mirror C's undefined/overflowing behavior.
        let v = tmp_a.wrapping_mul(tmp_b).wrapping_mul(c);

        // Print exactly "c v\n" with a single space and newline.
        println!("{} {}", c, v);
    }
}
