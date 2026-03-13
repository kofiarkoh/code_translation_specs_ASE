// Translated from the provided C program.
// Preserves behavior of the sieve and input/output formatting.
// Notes:
// - The original C code indexes up to `MAX` (<= MAX) but declared `char Prime[MAX];`
//   which would be out-of-bounds. To preserve the observable behavior of indexing
//   up to MAX, we allocate an array of length MAX+1 here.
// - The C loop bound used `i <= sqrt((double)MAX) + 1`. We compute the same bound.
// - Input values may be negative in C; the original program would print 0 for
//   negative n (the for loop `for(i=0;i<=n;i++)` executes zero times). We replicate that.
// - If an input n > MAX the original C program would have undefined behavior.
//   To avoid unsafety and panics in Rust we clamp such queries to MAX (this is the
//   most reasonable way to emulate a program intended to support indices up to MAX).
use std::io::{self, Read};

const MAX: usize = 1_000_000;

fn prime_search_eratos() -> Vec<u8> {
    // Allocate MAX+1 to allow indices 0..=MAX (the C code iterates j <= MAX).
    let mut prime = vec![1u8; MAX + 1];

    // Corresponding to: Prime[0] = 0; Prime[1] = 0;
    if MAX >= 0 {
        prime[0] = 0;
    }
    if MAX >= 1 {
        prime[1] = 0;
    }

    // Bound: i <= sqrt((double)MAX) + 1
    let limit = ((MAX as f64).sqrt() + 1.0) as usize;

    for i in 2..=limit {
        if prime[i] == 0 {
            continue;
        }
        let mut j = i * 2;
        while j <= MAX {
            prime[j] = 0;
            j += i;
        }
    }

    prime
}

fn main() {
    // Build sieve
    let prime = prime_search_eratos();

    // Build prefix sums so queries are O(1) (results identical to counting loop in C).
    let mut prefix = vec![0usize; MAX + 1];
    prefix[0] = if prime[0] != 0 { 1 } else { 0 };
    for i in 1..=MAX {
        prefix[i] = prefix[i - 1] + if prime[i] != 0 { 1 } else { 0 };
    }

    // Read all stdin, parse integers like repeated scanf("%d", &n) until EOF.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read failed");

    let mut out = String::new();

    for token in input.split_whitespace() {
        // Parse as i32 to handle negative values like the C program.
        match token.parse::<i32>() {
            Ok(n) => {
                if n < 0 {
                    out.push_str("0\n");
                } else {
                    let n_us = n as usize;
                    let cnt = if n_us <= MAX { prefix[n_us] } else { prefix[MAX] };
                    out.push_str(&format!("{}\n", cnt));
                }
            }
            Err(_) => {
                // Ignore non-integer tokens (scanf("%d") would fail; here we skip).
            }
        }
    }

    // Print results (matches printf("%d\n") per input line).
    print!("{}", out);
}
