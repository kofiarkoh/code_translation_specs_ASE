// translated from the provided C program
// Preserves semantics of the original algorithm.
// - Uses i32 to mirror C `int` (commonly 32-bit).
// - Uses wrapping_mul to emulate wraparound on multiplication as typically happens
//   on two's-complement machines (C signed overflow is UB, but most real-world
//   implementations exhibit two's-complement wrap; we emulate that).
// - The for-loop in C used a loop condition re-evaluating the (mutated) tmp_b each
//   iteration. We reproduce that by a while loop that checks `i <= tmp_b` each time.
//
// Note: The original C used `while (scanf("%d%d", &a, &b) != EOF)` which treats
// partial matches differently from strict token-pair reading. To avoid undefined
// / uninitialized behavior (which would be present in the C if only one integer
// was available), this Rust translation reads whitespace-separated tokens and
// processes complete pairs only. This keeps I/O formatting identical for valid
// input consisting of integer pairs.
use std::io::{self, Read};

fn main() {
    // Read entire stdin (matching scanf behavior of reading from stdin).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read stdin");

    let mut tokens = s.split_whitespace();

    // Process input as pairs of integers until tokens exhausted.
    while let Some(a_tok) = tokens.next() {
        // If there's no second token, break (we avoid using an uninitialized value).
        let b_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse tokens as i32. If parsing fails, this will panic (similar to a
        // runtime error for malformed input).
        let mut a: i32 = a_tok.parse().expect("parse int");
        let mut b: i32 = b_tok.parse().expect("parse int");

        // Swap if b > a, as in the C code.
        if b > a {
            let tmp = a;
            a = b;
            b = tmp;
        }

        let mut tmp_a = a;
        let mut tmp_b = b;
        let mut c: i32 = 1;

        // Reproduce the C for loop: for (int i = 2; i <= tmp_b; i++)
        // Note: since tmp_b is mutated inside the loop, the upper bound must
        // be re-evaluated each iteration — hence a while loop here.
        let mut i: i32 = 2;
        while i <= tmp_b {
            // Inner while: while(tmp_b%i==0 && tmp_a%i==0)
            while tmp_b % i == 0 && tmp_a % i == 0 {
                // Use wrapping_mul to emulate two's-complement wraparound.
                c = c.wrapping_mul(i);
                tmp_b = tmp_b / i;
                tmp_a = tmp_a / i;
            }
            i = i + 1;
        }

        // Compute tmp_a*tmp_b*c with wrapping multiplications to emulate C wrap.
        let prod = tmp_a.wrapping_mul(tmp_b).wrapping_mul(c);

        // Print exactly "%d %d\n"
        println!("{} {}", c, prod);
    }
}
