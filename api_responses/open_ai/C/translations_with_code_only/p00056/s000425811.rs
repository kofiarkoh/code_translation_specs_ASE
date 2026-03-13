// translated_from_c.rs
//
// Direct translation of the provided C program into Rust.
// Preserves the original algorithm and (intentional) quirks:
// - The sieve only marks composites up to 5000 (as in the C code).
// - The prime array size mirrors the C code (50001). Inputs > 50000 will panic
//   on indexing as they would be out-of-bounds in the original C program.
// The program reads integers from stdin until a 0 is encountered, and for each
// non-zero n prints the value computed by the original C program.
//
// Note: This is written in safe Rust and aims to preserve semantics of the C code.

use std::io::{self, Read};

fn set_sieve() -> Vec<u8> {
    // prime[i] == 0 means "considered prime" (as in the C code),
    // prime[i] == 1 means composite.
    let mut prime = vec![0u8; 50001]; // same size as C: int prime[50001];

    // mark even numbers starting at 4
    let mut i = 4usize;
    while i <= 5000 {
        prime[i] = 1;
        i += 2;
    }

    // sieve odd primes, but only up to 5000 as in the C code
    let mut i = 3usize;
    while i * i <= 5000 {
        if prime[i] == 0 {
            let mut o = i * 2;
            while o <= 5000 {
                prime[o] = 1;
                o += i;
            }
        }
        i += 2;
    }

    prime
}

fn main() {
    // Read all input tokens (like repeated scanf("%d", &num))
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let prime = set_sieve();

    let mut tokens = input.split_whitespace();
    // Emulate the C loop: read ints and stop when a 0 is encountered.
    while let Some(tok) = tokens.next() {
        // parse token as i32 to match typical scanf("%d")
        let num: i32 = match tok.parse() {
            Ok(n) => n,
            Err(_) => continue, // skip malformed tokens (scanf would fail)
        };

        if num == 0 {
            break;
        }

        // In the C code, count is set to 0 right before scanf each iteration.
        // Here we set it to 0 for each processed number.
        let mut count: i32 = 0;

        // For indexing into `prime` we need usize.
        // The original C program declared prime[50001], so valid indices are 0..=50000.
        // If num > 50000, the original C program would index out of bounds (UB).
        // This Rust translation will likewise panic on out-of-bounds to reflect that.
        let num_usize = num as usize;

        // Outer loop: for(i=2;i<=num;i++)
        let mut i = 2usize;
        while i <= num_usize {
            if prime.get(i).copied().unwrap_or(1) == 0 {
                // replicate the redundant check from the C code: if(i>num) break;
                if i > num_usize {
                    break;
                }
                // Inner loop: for(o=2;o<=num;o++)
                let mut o = 2usize;
                while o <= num_usize {
                    if prime.get(o).copied().unwrap_or(1) == 0 {
                        let sum = i + o;
                        if sum == num_usize {
                            count += 1;
                            break;
                        }
                        if sum > num_usize {
                            break;
                        }
                    }
                    o += 1;
                }
            }
            i += 1;
        }

        // print: printf("%d\n",count%2?(count/2+1):(count/2));
        let out = if count % 2 != 0 {
            count / 2 + 1
        } else {
            count / 2
        };
        println!("{}", out);
    }
}
