// translation of the provided C program to Rust
// Notes on semantics preservation:
// - We model C's 32-bit `int` with Rust's i32.
// - For arithmetic operations that in C might overflow, we use wrapping arithmetic to emulate
//   two's-complement wraparound (common behavior on modern architectures). C signed overflow is
//   formally UB, but many programs rely on wraparound; using wrapping_*() is a conservative choice.
// - Division is done on a wider i64 then cast back to i32 to avoid Rust panics on INT_MIN / -1;
//   this approximates the C computation of a/b (truncating toward zero) while avoiding Rust
//   runtime panics. Division by zero will still panic (approximating a runtime failure similar to C).
// - I/O behavior: read all whitespace-separated tokens from stdin, process in groups of 3 (a,b,n),
//   and print one line with the decimal `sum` per group, matching the C program's "%d\n" output.

use std::io::{self, Read};

fn main() {
    // Read entire stdin, split by whitespace, and iterate tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut tokens = input.split_whitespace();

    let mut out = String::new();

    loop {
        // Attempt to read a, b, n as in scanf("%d %d %d", &a, &b, &n)
        let a_tok = match tokens.next() {
            Some(t) => t,
            None => break, // EOF -> stop (matches behaviour of reading until EOF)
        };
        let b_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let n_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse tokens to i32. If parsing fails, propagate panic (scanf would fail/misbehave too).
        let mut a: i32 = a_tok.parse().expect("invalid integer");
        let b: i32 = b_tok.parse().expect("invalid integer");
        let n: i32 = n_tok.parse().expect("invalid integer");

        // Emulate: a -= a/b*b;
        // Compute quotient using i64 division (trunc toward zero), then cast to i32.
        // Multiplication and subtraction use wrapping arithmetic to emulate 32-bit wraparound.
        let q64 = (a as i64) / (b as i64); // will panic on division by zero (similar to C runtime error)
        let q = q64 as i32;
        let prod = q.wrapping_mul(b);
        a = a.wrapping_sub(prod);

        let mut sum: i32 = 0;

        // for(i=1;i<=n;i++) { ... }
        // If n <= 0 the loop body won't execute (matching C signed comparison).
        let mut i_val: i32 = 1;
        while i_val <= n {
            // a *= 10;
            a = a.wrapping_mul(10);

            // for(j=9;j>=0;j--) { if(b*j<=a) break; }
            // We emulate possible wraparound in b*j via wrapping_mul.
            let mut j: i32 = 9;
            loop {
                let prod_bj = b.wrapping_mul(j);
                if prod_bj <= a {
                    break;
                }
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            // a -= b*j;
            let prod_bj = b.wrapping_mul(j);
            a = a.wrapping_sub(prod_bj);

            // sum += j;
            sum = sum.wrapping_add(j);

            i_val += 1;
        }

        // Print sum with newline, matching printf("%d\n", sum);
        out.push_str(&format!("{}\n", sum));
    }

    // Write all output at once
    print!("{}", out);
}
