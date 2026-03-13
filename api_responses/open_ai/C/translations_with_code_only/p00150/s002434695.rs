// main.rs
// Translated from the provided C program.
// Notes:
// - The original C declared `char prime[10001];` but called setprime(10001),
//   which writes up to index 10001. To preserve the intended behavior and avoid
//   out-of-bounds access, we allocate `MAX+1` entries so indices 0..=MAX are valid.
// - The array stores 0 for "prime" and 1 for "composite" to match the C code.
// - I/O behavior (reading ints until 0, printing two ints per line) is preserved.

use std::io::{self, Read};

const MAX: usize = 10001;

fn setprime(prime: &mut [u8], max: usize) {
    // Mark even numbers >= 4 as composite
    let mut i = 4usize;
    while i <= max {
        prime[i] = 1;
        i += 2;
    }

    // Sieve: for odd i from 3 while i*i <= max
    let mut i = 3usize;
    while i * i <= max {
        if prime[i] == 0 {
            // mark multiples of i starting from i*2 (matches the C code exactly)
            let mut j = i * 2;
            while j <= max {
                prime[j] = 1;
                j += i;
            }
        }
        i += 2;
    }
}

fn main() {
    // Allocate one extra element so index MAX (10001) is valid,
    // matching the behavior of calling setprime(10001) in the C code.
    let mut prime = vec![0u8; MAX + 1];

    setprime(&mut prime, MAX);

    // Read all input and parse integers
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut out = String::new();

    for token in input.split_whitespace() {
        // Parse token as integer; ignore parse errors (none expected for valid input)
        if let Ok(n0) = token.parse::<i32>() {
            if n0 == 0 {
                break;
            }
            // In the original C code, `n` is an int and used as an index into prime[].
            // We assume inputs are within the intended bounds (<= MAX). If an input
            // exceeds MAX, to avoid panics we treat it as MAX (this is a pragmatic
            // safety measure; typical inputs for the original program are <= 10001).
            let mut n = if n0 < 0 { 0 } else { n0 as usize };
            if n > MAX {
                n = MAX;
            }

            let mut i = n;
            while i >= 7 {
                // access prime[i] and prime[i-2]; these indices are valid because we
                // allocated up to MAX and clamped n above.
                if prime[i] + prime[i - 2] == 0 {
                    out.push_str(&format!("{} {}\n", i - 2, i));
                    break;
                }
                i -= 1;
            }
        }
    }

    // Preserve stdout formatting exactly (each result line ends with '\n').
    print!("{}", out);
}
