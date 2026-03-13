// translated from the given C program.
// - Uses i32 for `n` and loop variable `i` to match C `int` (typically 32-bit).
// - Uses i64 for `sum` to match C `long long` (typically 64-bit).
// - Emulates two's-complement wraparound for integer operations using wrapping_* to approximate common C implementation behavior.
// - Reads whitespace-separated integers from stdin until EOF and prints one line per input integer, matching the original printf("%lld\n", sum).
use std::io::{self, Read};

fn main() {
    // Read entire stdin as sequence of whitespace-separated tokens (like repeated scanf("%d", &n))
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // If token is not a valid i32, break (typical judge inputs are valid integers).
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // sum corresponds to C's `long long int sum;`
        let mut sum: i64 = 0;

        // limit = 600 - n (computed in i32 as in C)
        let limit: i32 = 600i32.wrapping_sub(n);

        // i starts at n and increments by n each iteration (using wrapping_add to emulate two's-complement behavior)
        let mut i: i32 = n;

        // Replicate: for (i = n; i <= 600 - n; i += n) { sum += i * i; }
        loop {
            if !(i <= limit) {
                break;
            }
            let ii = i as i64;
            // Compute square in i64 to avoid undefined behavior from 32-bit signed overflow;
            // use wrapping_mul and wrapping_add on i64 to emulate typical machine wraparound for long long.
            sum = sum.wrapping_add(ii.wrapping_mul(ii));
            i = i.wrapping_add(n);
        }

        // sum *= n; with wrapping multiplication to emulate common two's-complement behavior
        sum = sum.wrapping_mul(n as i64);

        out.push_str(&format!("{}\n", sum));
    }

    // Print accumulated output (preserves exact newline formatting)
    print!("{}", out);
}
