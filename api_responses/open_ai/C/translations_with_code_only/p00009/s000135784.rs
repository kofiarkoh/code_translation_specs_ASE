use std::io::{self, Read};

const MAX: usize = 1_000_000;

fn main() {
    // Allocate MAX+1 to match the C code's usage of indices up to and including MAX.
    // The original C code declared `char Prime[MAX];` but then accessed index `MAX`,
    // which would be out-of-bounds. To preserve the intended behavior (and avoid UB),
    // we keep indices 0..=MAX valid here.
    let mut prime = vec![1u8; MAX + 1];

    // Corresponds to: Prime[0] = 0; Prime[1] = 0;
    prime[0] = 0;
    if MAX >= 1 {
        prime[1] = 0;
    }

    // Compute limit = floor(sqrt(MAX) + 1) to match the C loop condition:
    // for (i = 2; i <= sqrt((double)MAX) + 1; i++)
    let limit = ((MAX as f64).sqrt() + 1.0).floor() as usize;

    for i in 2..=limit {
        if prime[i] == 0 {
            continue;
        }
        // j starts at i*2 and goes up to and including MAX, stepping by i.
        let mut j = i * 2;
        while j <= MAX {
            prime[j] = 0;
            j += i;
        }
    }

    // Read all input and parse integers. The C program repeatedly called scanf("%d", &n)
    // until EOF and processed each integer. We emulate that by reading whitespace-separated
    // tokens and parsing them as i32 values.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin read error");

    for token in input.split_whitespace() {
        // Parse token as i32. If parsing fails, skip the token.
        // (Typical inputs for the original program are integers.)
        let n: i32 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Count primes for i = 0 ..= n as in the C code's loop:
        // for (i = 0; i <= n; i++) { if (Prime[i]) Count++; }
        // If n < 0 the loop does not run and Count stays 0.
        let count = if n < 0 {
            0usize
        } else {
            let up = n as usize;
            let lim = if up > MAX { MAX } else { up };
            let mut c = 0usize;
            for i in 0..=lim {
                if prime[i] != 0 {
                    c += 1;
                }
            }
            c
        };

        println!("{}", count);
    }
}
