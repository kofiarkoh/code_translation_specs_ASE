// Translated from the given C program, preserving semantics.
// - Uses a manual byte-level parser to emulate scanf + getchar behavior:
//   scanf("%lf", &x) consumes a floating token but leaves the following char,
//   then getchar() consumes exactly one char (which may be space/newline).
// - DP and bit-twiddling follow the original exactly. DP table initialized
//   to -INF (-100_000_000.0) and memoization checks dp >= 0.0 as in C.
// - Floating expressions are computed the same way (no algebraic simplification).
//
// Note: This is safe, idiomatic Rust; no `unsafe` is used.

use std::io::{self, Read};
use std::str::FromStr;

const INF: f64 = 100000000.0;
const EPS: f64 = 1e-5;
const MAX_N: usize = 12;

fn main() {
    // Read entire stdin into a buffer to allow peeking/consuming single chars
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut pos: usize = 0;
    let len_input = bytes.len();

    // Helper to skip ASCII whitespace (matches C's isspace behavior sufficiently for this task)
    fn is_space(b: u8) -> bool {
        matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C)
    }

    // Parse next f64 similarly to scanf("%lf", ...):
    // skip leading whitespace, then take a contiguous run of characters that could be
    // part of a floating literal ("+-0123456789.eE") and parse them.
    fn parse_next_f64(bytes: &[u8], pos: &mut usize) -> Option<f64> {
        let n = bytes.len();
        // skip leading whitespace
        while *pos < n && is_space(bytes[*pos]) {
            *pos += 1;
        }
        if *pos >= n {
            return None;
        }
        let start = *pos;
        // accept chars that could be part of a float literal
        while *pos < n {
            match bytes[*pos] {
                b'+' | b'-' | b'0'..=b'9' | b'.' | b'e' | b'E' => {
                    *pos += 1;
                }
                _ => break,
            }
        }
        if start == *pos {
            return None;
        }
        // SAFETY: the slice contains ASCII characters only in the accepted set
        let s = std::str::from_utf8(&bytes[start..*pos]).ok()?;
        f64::from_str(s).ok()
    }

    // Storage for cakes and dp
    let mut cake = [0.0_f64; MAX_N];
    // dp as flat vector: size (1<<12) * 12
    let dp_size = (1 << MAX_N) * MAX_N;
    let mut dp = vec![ -INF; dp_size ];

    // The recursive tsp function; uses &mut dp and cake captured from outer scope.
    // dp indexed as dp[bit * 12 + now]
    fn tsp_rec(
        bit: usize,
        now: usize,
        n: usize,
        cake: &[f64; MAX_N],
        dp: &mut [f64],
    ) -> f64 {
        let idx = bit * MAX_N + now;
        if dp[idx] >= 0.0 {
            return dp[idx];
        }

        if bit == ((1usize << n) - 1) && now == 0 {
            dp[idx] = 0.0;
            return 0.0;
        }

        let mut res = INF;
        for i in 0..n {
            if ((bit >> i) & 1) == 0 {
                // compute sqrt((a+b)^2 - (a-b)^2) as in C code (no algebraic simplification)
                let a = cake[i];
                let b = cake[now];
                let term = (a + b) * (a + b) - (a - b) * (a - b);
                let d = term.sqrt();
                let candidate = tsp_rec(bit | (1 << i), i, n, cake, dp) + d;
                if candidate < res {
                    res = candidate;
                }
            }
        }

        dp[idx] = res;
        res
    }

    // Main loop: emulate while (scanf("%lf", &len) != EOF) { ... }
    while let Some(len_val) = parse_next_f64(bytes, &mut pos) {
        // emulate the getchar() immediately after scanf("%lf", &len)
        if pos < len_input {
            // consume exactly one char
            pos += 1;
        } else {
            // if EOF, just continue; this mirrors that there's nothing to consume
        }

        let mut n: usize = 0;
        // Read cake radii until a consumed separator char is '\n'
        loop {
            if n >= MAX_N {
                // In C the array is size 12; reading more would overflow.
                // We'll stop reading further numbers to preserve behavior within bounds.
                // (C would overwrite memory; here we stop at capacity.)
                break;
            }
            match parse_next_f64(bytes, &mut pos) {
                Some(v) => {
                    cake[n] = v;
                    n += 1;
                }
                None => {
                    // no more number to read
                    break;
                }
            }
            // emulate getchar() after each cake scanf
            let consumed_char = if pos < len_input {
                let c = bytes[pos];
                pos += 1;
                Some(c)
            } else {
                None
            };
            if consumed_char == Some(b'\n') {
                break;
            }
            // if consumed_char is None (EOF), break to avoid infinite loop
            if consumed_char.is_none() {
                break;
            }
            // else continue reading next cake
        }

        // Initialize dp to -INF for all entries (as in the C code)
        for v in dp.iter_mut() {
            *v = -INF;
        }

        // If n is 0, behavior: tsp(0,0) will reference cake[0], but original C would have read at least one cake.
        // We'll guard: if n == 0, treat as NA (no cake), but follow C's likely assumption of n>0.
        let result = if n == 0 {
            INF // large value -> NA
        } else {
            tsp_rec(0, 0, n, &cake, &mut dp)
        };

        if result < len_val + EPS {
            println!("OK");
        } else {
            println!("NA");
        }
    }
}
