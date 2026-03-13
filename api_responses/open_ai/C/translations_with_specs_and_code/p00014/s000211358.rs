use std::io::{self, Read, Write};

fn main() {
    // Read entire stdin as whitespace-separated tokens, analogous to repeated scanf("%d", &n).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, just exit (matches program ending without printing).
        return;
    }

    let mut out = String::new();

    for token in input.split_whitespace() {
        // Per the SPEC, inputs are expected to be valid decimal integers (C `int`).
        // We parse into i32 to mirror C `int` and then use i64 for accumulation.
        let n_i32: i32 = match token.parse() {
            Ok(v) => v,
            // The original C program does not handle non-numeric tokens (scanf would return 0
            // and lead to undefined behavior). Here we stop processing further tokens if parsing fails.
            // This choice follows the SPEC assumption that inputs are well-formed integers.
            Err(_) => break,
        };

        let n = n_i32 as i64;
        let mut sum: i64 = 0;

        // Bound: i <= 600 - n (all in i64)
        let bound = 600i64 - n;

        // Use the same loop structure as the C program:
        // for (i = n; i <= 600 - n; i += n) { sum += i * i; }
        //
        // Note: If n == 0 or n < 0 this loop may not terminate, matching the SPEC which
        // documents that such inputs are invalid / lead to non-terminating behavior.
        let mut i = n;
        while i <= bound {
            // Use wrapping arithmetic to avoid debug-build panics on overflow.
            // The SPEC states the original program does not check for overflow.
            sum = sum.wrapping_add(i.wrapping_mul(i));
            i = i.wrapping_add(n);
        }

        sum = sum.wrapping_mul(n);

        // Append result followed by a newline, matching printf("%lld\n", sum);
        out.push_str(&format!("{}\n", sum));
    }

    // Write all output at once.
    let _ = io::stdout().write_all(out.as_bytes());
}
